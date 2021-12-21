use crate::{
    dependencies::api,
    js::common::{parse_bridge_pars, to_bridge_res},
    service::{constants::PRECISION, str_to_algos::microalgos_to_algos_str},
};
use anyhow::{Error, Result};
use core::{
    decimal_util::{AsDecimal, DecimalExt},
    dependencies::algod,
    flows::{
        harvest::harvest::investor_can_harvest_amount_calc,
        withdraw::withdraw::{FIXED_FEE, MIN_BALANCE},
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
    let api = api();

    let project = api.load_project(&pars.project_id).await?;

    let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

    let investor_state =
        central_investor_state(&algod, investor_address, project.central_app_id).await?;
    let central_state = central_global_state(&algod, project.central_app_id).await?;

    let customer_escrow_balance = algod
        .account_information(&project.customer_escrow.address)
        .await?
        .amount;

    // TODO review redundancy with backend, as we store the share count in the db too
    // maybe we shouldn't store them in the backend (also meaning: the backend can't deliver Project objects but a reduced view of them),
    // as it may get out of sync when shares are diluted
    // also use Decimal for everything involving fractions
    let investor_percentage =
        investor_state.shares.as_decimal() / project.specs.shares.count.as_decimal();

    let withdrawable_customer_escrow_amount = customer_escrow_balance - (MIN_BALANCE + FIXED_FEE);
    // This is basically "simulate that the customer escrow was already drained"
    // we use this value, as harvesting will drain the customer escrow if it has a balance (> MIN_BALANCE + FIXED_FEE)
    // and the draining step is invisible to the user (aside of adding more txs to the harvesting txs to sign)
    let received_total_including_customer_escrow_balance =
        central_state.received + withdrawable_customer_escrow_amount;

    let can_harvest = investor_can_harvest_amount_calc(
        received_total_including_customer_escrow_balance,
        investor_state.harvested,
        investor_state.shares,
        project.specs.shares.count,
        PRECISION,
        project.specs.investors_share,
    );

    log::info!("Determined harvest amount: {}, from central_received_total: {}, withdrawable_customer_escrow_amount: {}, investor_shares_count: {}, share supply: {}", can_harvest, central_state.received, withdrawable_customer_escrow_amount, investor_state.shares, project.specs.shares.count);

    Ok(LoadInvestmentResJs {
        investor_shares_count: investor_state.shares.to_string(),
        investor_percentage: investor_percentage.format_percentage(),
        investor_already_retrieved_amount: microalgos_to_algos_str(investor_state.harvested),
        investor_harvestable_amount: microalgos_to_algos_str(can_harvest),
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
    investor_already_retrieved_amount: String,
    investor_harvestable_amount: String,
    investor_harvestable_amount_microalgos: String, // passthrough
}
