use crate::calculate_profit_percentage;
use crate::provider::investment_provider::{
    CalcPriceAndPercSpecs, InvestmentProvider, LoadInvestmentParJs, LoadInvestmentResJs,
};
use crate::{
    dependencies::{api, capi_deps, funds_asset_specs},
    service::{constants::PRECISION, str_to_algos::base_units_to_display_units_str},
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::state::account_state::asset_holdings;
use base::state::dao_shares::dao_shares_with_dao_state;
use base::{
    decimal_util::DecimalExt,
    dependencies::algod,
    flows::{
        claim::claim::claimable_dividend,
        create_dao::{share_amount::ShareAmount, storage::load_dao::load_dao},
        drain::drain::drain_amounts,
    },
    funds::FundsAmount,
    state::{
        app_state::ApplicationLocalStateError,
        dao_app_state::{dao_global_state, dao_investor_state},
    },
};

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

        let investor_state_res = dao_investor_state(&algod, investor_address, dao.app_id).await;
        let (investor_locked_shares, investor_claimed, already_retrieved) = match investor_state_res
        {
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

        let central_state = dao_global_state(&algod, dao.app_id).await?;
        let dao_shares =
            dao_shares_with_dao_state(&algod, dao_id.0, dao.shares_asset_id, &central_state)
                .await?;

        let investor_locked_percentage =
            investor_locked_shares.as_decimal() / dao.specs.shares.supply.as_decimal();

        let drain_amounts = drain_amounts(
            &algod,
            capi_deps.escrow_percentage,
            funds_asset_specs.id,
            dao.customer_escrow.address(),
        )
        .await?;
        let withdrawable_customer_escrow_amount = drain_amounts.dao;
        // This is basically "simulate that the customer escrow was already drained"
        // we use this value, as harvesting will drain the customer escrow if it has a balance (> MIN_BALANCE + FIXED_FEE)
        // and the draining step is invisible to the user (aside of adding more txs to the claiming txs to sign)
        let received_total_including_customer_escrow_balance =
            central_state.received + withdrawable_customer_escrow_amount;

        let can_claim = claimable_dividend(
            received_total_including_customer_escrow_balance,
            investor_claimed,
            dao.specs.shares.supply,
            investor_locked_shares,
            PRECISION,
            dao.specs.investors_share,
        )?;

        let investor_percentage_relative_to_total =
            investor_locked_percentage * dao.specs.investors_share.value();

        log::info!("Determined claim amount: {}, from central_received_total: {}, withdrawable_customer_escrow_amount: {}, investor_shares_count: {}, share supply: {}", can_claim, central_state.received, withdrawable_customer_escrow_amount, investor_locked_shares, dao.specs.shares.supply);

        let investor_holdings =
            asset_holdings(&algod, investor_address, dao.shares_asset_id).await?;

        let one_share_profit_percentage = calculate_profit_percentage(
            ShareAmount::new(1),
            dao.specs.shares.supply,
            dao.specs.investors_share,
        )?;

        Ok(LoadInvestmentResJs {
            investor_shares_count: investor_locked_shares.to_string(),

            investor_percentage: investor_locked_percentage.format_percentage(),
            investor_percentage_number: investor_locked_percentage.to_string(),
            investor_percentage_relative_to_total_number: investor_percentage_relative_to_total
                .to_string(),

            investor_already_retrieved_amount: base_units_to_display_units_str(
                already_retrieved,
                &funds_asset_specs,
            ),
            investor_claimable_dividend: base_units_to_display_units_str(
                can_claim,
                &funds_asset_specs,
            ),
            investor_claimable_dividend_microalgos: can_claim.to_string(),
            available_shares: dao_shares.available.to_string(),
            investor_locked_shares: investor_locked_shares.to_string(),
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
