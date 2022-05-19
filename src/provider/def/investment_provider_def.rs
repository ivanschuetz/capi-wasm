use crate::calculate_profit_percentage;
use crate::dependencies::FundsAssetSpecs;
use crate::provider::investment_provider::{
    CalcPriceAndPercSpecs, InvestmentProvider, LoadInvestmentParJs, LoadInvestmentResJs,
};
use crate::{
    dependencies::{api, capi_deps, funds_asset_specs},
    service::{constants::PRECISION, str_to_algos::base_units_to_display_units_str},
};
use algonaut::algod::v2::Algod;
use algonaut::core::Address;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::capi_deps::CapiAssetDaoDeps;
use base::flows::create_dao::model::Dao;
use base::flows::{
    claim::claim::claimable_dividend, create_dao::storage::load_dao::load_dao,
    drain::drain::drain_amounts,
};
use base::state::account_state::asset_holdings;
use base::state::dao_shares::dao_shares_with_dao_state;
use mbase::dependencies::algod;
use mbase::models::dao_app_id::DaoAppId;
use mbase::models::funds::FundsAmount;
use mbase::models::share_amount::ShareAmount;
use mbase::state::app_state::ApplicationLocalStateError;
use mbase::state::dao_app_state::{dao_global_state, dao_investor_state, CentralAppGlobalState};
use mbase::util::decimal_util::DecimalExt;

pub struct InvestmentProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl InvestmentProvider for InvestmentProviderDef {
    // TODO parallelize requests if possible
    async fn get(&self, pars: LoadInvestmentParJs) -> Result<LoadInvestmentResJs> {
        log::debug!("bridge_load_investment, pars: {:?}", pars);

        let algod = algod();
        let api = api();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

        let investor_view_data =
            investor_local_state_view_data(&algod, investor_address, dao.app_id).await?;

        // The % of investor's locked shares relative to the total supply
        let investor_locked_percentage_of_supply =
            investor_view_data.locked_shares.as_decimal() / dao.specs.shares.supply.as_decimal();
        // The % of DAO income the investor is entitled to, based on their locked shares
        let investor_dividend_percentage =
            investor_locked_percentage_of_supply * dao.specs.investors_share.value();

        let central_state = dao_global_state(&algod, dao.app_id).await?;

        let claimable_dividend = fetch_claimable_dividend(
            &algod,
            investor_view_data.claimed,
            investor_view_data.locked_shares,
            PRECISION,
            &dao,
            &capi_deps,
            &funds_asset_specs,
            &central_state,
        )
        .await?;

        let dao_shares =
            dao_shares_with_dao_state(&algod, dao_id.0, dao.shares_asset_id, &central_state)
                .await?;

        let investor_holdings =
            asset_holdings(&algod, investor_address, dao.shares_asset_id).await?;

        let one_share_profit_percentage = calculate_profit_percentage(
            ShareAmount::new(1),
            dao.specs.shares.supply,
            dao.specs.investors_share,
        )?;

        Ok(LoadInvestmentResJs {
            investor_shares_count: investor_view_data.locked_shares.to_string(),

            // TODO do we still need these? don't we care only about investor_percentage_relative_to_total (the % of income basically)?,
            // this is the percentage relative only to the total asset supply (i.e. other investors)
            investor_percentage: investor_locked_percentage_of_supply.format_percentage(),
            investor_percentage_number: investor_locked_percentage_of_supply.to_string(),

            // TODO rename field in investor_dividend_percentage
            investor_percentage_relative_to_total_number: investor_dividend_percentage.to_string(),

            investor_already_retrieved_amount: base_units_to_display_units_str(
                investor_view_data.retrieved,
                &funds_asset_specs,
            ),
            investor_claimable_dividend: base_units_to_display_units_str(
                claimable_dividend,
                &funds_asset_specs,
            ),
            investor_claimable_dividend_microalgos: claimable_dividend.to_string(),
            available_shares: dao_shares.available.to_string(),
            investor_locked_shares: investor_view_data.locked_shares.to_string(),
            investor_unlocked_shares: investor_holdings.to_string(),

            init_share_price: base_units_to_display_units_str(
                central_state.share_price,
                &funds_asset_specs,
            ),
            init_profit_percentage: one_share_profit_percentage.format_percentage(),

            share_specs_msg_pack: rmp_serde::to_vec_named(&CalcPriceAndPercSpecs {
                funds_specs: funds_asset_specs,
            })?,
        })
    }
}

/// Returns a "for ui" version of the local state (essentially: defaults things to 0 if the investor is not opted in)
pub async fn investor_local_state_view_data(
    algod: &Algod,
    investor_address: &Address,
    app_id: DaoAppId,
) -> Result<InvestorLocalStateViewData> {
    let investor_state_res = dao_investor_state(algod, investor_address, app_id).await;
    let (locked_shares, claimed, retrieved) = match investor_state_res {
        Ok(state) => (
            state.shares,
            state.claimed,
            state.claimed - state.claimed_init,
        ),
        Err(e) => {
            if e == ApplicationLocalStateError::NotOptedIn {
                // If the investor isn't opted in (unlocked the shares - note that currently it's not possible to unlock only a part of the shares),
                // we don't show an error, it just means that they've 0 shares and haven't claimed anything.
                // the later is discussable UX wise (they may have claimed before unlocking the shares),
                // but the local state is deleted when unlocking (opting out), so 0 is the only meaningful thing we can return here.
                (
                    ShareAmount::new(0),
                    FundsAmount::new(0),
                    FundsAmount::new(0),
                )
            } else {
                Err(e)?
            }
        }
    };

    Ok(InvestorLocalStateViewData {
        locked_shares,
        claimed,
        retrieved,
    })
}

#[derive(Debug, Clone)]
pub struct InvestorLocalStateViewData {
    pub locked_shares: ShareAmount,
    pub claimed: FundsAmount,
    pub retrieved: FundsAmount,
}

#[allow(clippy::too_many_arguments)]
pub async fn fetch_claimable_dividend(
    algod: &Algod,

    investor_claimed: FundsAmount,
    investor_locked_shares: ShareAmount,

    precision: u64,

    dao: &Dao,

    capi_deps: &CapiAssetDaoDeps,
    funds_specs: &FundsAssetSpecs,

    app_state: &CentralAppGlobalState,
) -> Result<FundsAmount> {
    let drain_amounts = drain_amounts(
        algod,
        capi_deps.escrow_percentage,
        funds_specs.id,
        dao.customer_escrow.address(),
    )
    .await?;

    let withdrawable_customer_escrow_amount = drain_amounts.dao;

    // This is basically "simulate that the customer escrow was already drained"
    // we use this value, as harvesting will drain the customer escrow if it has a balance (> MIN_BALANCE + FIXED_FEE)
    // and the draining step is invisible to the user (aside of adding more txs to the claiming txs to sign)
    let received_total_including_customer_escrow_balance =
        app_state.received + withdrawable_customer_escrow_amount;

    let can_claim = claimable_dividend(
        received_total_including_customer_escrow_balance,
        investor_claimed,
        dao.specs.shares.supply,
        investor_locked_shares,
        precision,
        dao.specs.investors_share,
    )?;

    Ok(can_claim)
}
