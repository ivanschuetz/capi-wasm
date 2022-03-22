use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    js::common::{parse_bridge_pars, to_bridge_res},
    service::{constants::PRECISION, str_to_algos::base_units_to_display_units_str},
    teal::programs,
};
use anyhow::{anyhow, Error, Result};
use core::{
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
        central_app_state::{dao_global_state, dao_investor_state},
    },
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_investment(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_investment, pars: {:?}", pars);
    to_bridge_res(_bridge_load_investment(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_investment(pars: LoadInvestmentParJs) -> Result<LoadInvestmentResJs> {
    log::debug!("bridge_load_investment, pars: {:?}", pars);

    let algod = algod();
    let funds_asset_specs = funds_asset_specs();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &programs.escrows, &capi_deps).await?;

    let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

    let investor_state_res = dao_investor_state(&algod, investor_address, dao.app_id).await;
    let (investor_shares, investor_claimed) = match investor_state_res {
        Ok(state) => (state.shares, state.claimed),
        Err(e) => {
            if e == ApplicationLocalStateError::NotOptedIn {
                // If the investor isn't opted in (unlocked the shares - note that currently it's not possible to unlock only a part of the shares),
                // we don't show an error, it just means that they've 0 shares and haven't claimed anything.
                // the later is discussable UX wise (they may have claimed before unlocking the shares),
                // but the local state is deleted when unlocking (opting out), so 0 is the only meaningful thing we can return here.
                (ShareAmount::new(0), FundsAmount::new(0))
            } else {
                Err(e)?
            }
        }
    };

    let central_state = dao_global_state(&algod, dao.app_id).await?;

    let investor_percentage = investor_shares.as_decimal() / dao.specs.shares.supply.as_decimal();

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
        investor_shares,
        PRECISION,
        dao.specs.investors_part(),
    )?;

    let investors_share_normalized = dao
        .specs
        .investors_part()
        .as_decimal()
        .checked_div(100u8.into())
        .ok_or_else(|| anyhow!("Unexpected: dividing returned None"))?;
    let investor_percentage_relative_to_total = investor_percentage * investors_share_normalized;

    log::info!("Determined claim amount: {}, from central_received_total: {}, withdrawable_customer_escrow_amount: {}, investor_shares_count: {}, share supply: {}", can_claim, central_state.received, withdrawable_customer_escrow_amount, investor_shares, dao.specs.shares.supply);

    Ok(LoadInvestmentResJs {
        investor_shares_count: investor_shares.to_string(),

        investor_percentage: investor_percentage.format_percentage(),
        investor_percentage_number: investor_percentage.to_string(),
        investor_percentage_relative_to_total_number: investor_percentage_relative_to_total
            .to_string(),

        investors_share_number: investors_share_normalized.to_string(),

        investor_already_retrieved_amount: base_units_to_display_units_str(
            investor_claimed,
            &funds_asset_specs,
        ),
        investor_claimable_dividend: base_units_to_display_units_str(can_claim, &funds_asset_specs),
        investor_claimable_dividend_microalgos: can_claim.to_string(),
    })
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct LoadInvestmentParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadInvestmentResJs {
    investor_shares_count: String,
    investor_percentage: String,
    investor_percentage_number: String, // relative to investor's share (part reserved to investors)
    investor_percentage_relative_to_total_number: String, // relative to all the dao's income
    investors_share_number: String, // from Dao - copied here just for convenience (to retrieve all the display data from this struct)
    investor_already_retrieved_amount: String,
    investor_claimable_dividend: String,
    investor_claimable_dividend_microalgos: String, // passthrough
}
