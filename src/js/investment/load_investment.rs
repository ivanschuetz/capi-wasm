use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    js::common::{parse_bridge_pars, to_bridge_res},
    service::{constants::PRECISION, str_to_algos::base_units_to_display_units_str},
    teal::programs,
};
use anyhow::{anyhow, Error, Result};
use core::{
    decimal_util::DecimalExt,
    dependencies::{algod, indexer},
    flows::{
        create_project::storage::load_project::load_project, drain::drain::drain_amounts,
        harvest::harvest::investor_can_harvest_amount_calc,
    },
    state::central_app_state::{central_global_state, central_investor_state},
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
    let indexer = indexer();
    let funds_asset_specs = funds_asset_specs();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs.escrows, &capi_deps)
        .await?
        .project;

    let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

    let investor_state =
        central_investor_state(&algod, investor_address, project.central_app_id).await?;
    let central_state = central_global_state(&algod, project.central_app_id).await?;

    // TODO review redundancy with backend, as we store the share count in the db too
    // maybe we shouldn't store them in the backend (also meaning: the backend can't deliver Project objects but a reduced view of them),
    // as it may get out of sync when shares are diluted
    // also use Decimal for everything involving fractions
    let investor_percentage =
        investor_state.shares.as_decimal() / project.specs.shares.supply.as_decimal();

    let drain_amounts = drain_amounts(
        &algod,
        capi_deps.escrow_percentage,
        funds_asset_specs.id,
        project.customer_escrow.address(),
    )
    .await?;
    let withdrawable_customer_escrow_amount = drain_amounts.dao;
    // This is basically "simulate that the customer escrow was already drained"
    // we use this value, as harvesting will drain the customer escrow if it has a balance (> MIN_BALANCE + FIXED_FEE)
    // and the draining step is invisible to the user (aside of adding more txs to the harvesting txs to sign)
    let received_total_including_customer_escrow_balance =
        central_state.received + withdrawable_customer_escrow_amount;

    let can_harvest = investor_can_harvest_amount_calc(
        received_total_including_customer_escrow_balance,
        investor_state.harvested,
        investor_state.shares,
        project.specs.shares.supply,
        PRECISION,
        project.specs.investors_part(),
    );

    let investors_share_normalized = project
        .specs
        .investors_part()
        .as_decimal()
        .checked_div(100u8.into())
        .ok_or_else(|| anyhow!("Unexpected: dividing returned None"))?;
    let investor_percentage_relative_to_total = investor_percentage * investors_share_normalized;

    log::info!("Determined harvest amount: {}, from central_received_total: {}, withdrawable_customer_escrow_amount: {}, investor_shares_count: {}, share supply: {}", can_harvest, central_state.received, withdrawable_customer_escrow_amount, investor_state.shares, project.specs.shares.supply);

    Ok(LoadInvestmentResJs {
        investor_shares_count: investor_state.shares.to_string(),

        investor_percentage: investor_percentage.format_percentage(),
        investor_percentage_number: investor_percentage.to_string(),
        investor_percentage_relative_to_total_number: investor_percentage_relative_to_total
            .to_string(),

        investors_share_number: investors_share_normalized.to_string(),

        investor_already_retrieved_amount: base_units_to_display_units_str(
            investor_state.harvested,
            &funds_asset_specs,
        ),
        investor_harvestable_amount: base_units_to_display_units_str(
            can_harvest,
            &funds_asset_specs,
        ),
        investor_harvestable_amount_microalgos: can_harvest.to_string(),
    })
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct LoadInvestmentParJs {
    pub project_id: String,
    // TODO remove, central id in project (we fetch it here)
    pub app_id: String,
    pub shares_asset_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadInvestmentResJs {
    investor_shares_count: String,
    investor_percentage: String,
    investor_percentage_number: String, // relative to investor's share (part reserved to investors)
    investor_percentage_relative_to_total_number: String, // relative to all the project's income
    investors_share_number: String, // from Project - copied here just for convenience (to retrieve all the display data from this struct)
    investor_already_retrieved_amount: String,
    investor_harvestable_amount: String,
    investor_harvestable_amount_microalgos: String, // passthrough
}
