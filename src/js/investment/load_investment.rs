use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
    service::{
        app_state::{
            central_received_total, harvested_total_from_local_vars,
            investor_can_harvest_amount_calc, local_vars, owned_shares_count_from_local_vars,
        },
        str_to_algos::microalgos_to_algos,
    },
};
use anyhow::{Error, Result};
use make::flows::withdraw::logic::{FIXED_FEE, MIN_BALANCE};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_investment(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_investment, pars: {:?}", pars);
    to_bridge_res(_bridge_load_investment(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_investment(pars: LoadInvestmentParJs) -> Result<LoadInvestmentResJs> {
    log::debug!("bridge_load_investment, pars: {:?}", pars);

    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let app_id = pars.app_id.parse()?;

    let app_local_vars = local_vars(
        &algod,
        &pars.investor_address.parse().map_err(Error::msg)?,
        project.central_app_id,
    )
    .await?;

    let customer_escrow_balance = algod
        .account_information(&project.customer_escrow.address)
        .await?
        .amount;
    let investor_shares_count = owned_shares_count_from_local_vars(&app_local_vars).await?;

    // TODO review redundancy with backend, as we store the share count in the db too
    // maybe we shouldn't store them in the backend (also meaning: the backend can't deliver Project objects but a reduced view of them),
    // as it may get out of sync when shares are diluted
    // also use Decimal for everything involving fractions
    let investor_percentage = investor_shares_count as f64 / project.specs.shares.count as f64;

    let central_received_total = central_received_total(&algod, app_id).await?;
    let already_harvested = harvested_total_from_local_vars(&app_local_vars).await?;

    let withdrawable_customer_escrow_amount = customer_escrow_balance - (MIN_BALANCE + FIXED_FEE);
    // This is basically "simulate that the customer escrow was already drained"
    // we use this value, as harvesting will drain the customer escrow if it has a balance (> MIN_BALANCE + FIXED_FEE)
    // and the draining step is invisible to the user (aside of adding more txs to the harvesting txs to sign)
    let received_total_including_customer_escrow_balance =
        central_received_total + withdrawable_customer_escrow_amount;

    let can_harvest = investor_can_harvest_amount_calc(
        received_total_including_customer_escrow_balance,
        already_harvested,
        investor_shares_count,
        project.specs.shares.count,
    );
    Ok(LoadInvestmentResJs {
        investor_shares_count: investor_shares_count.to_string(),
        investor_percentage: format!("{} %", (investor_percentage * 100 as f64).to_string()),
        investor_already_retrieved_amount: microalgos_to_algos(already_harvested).to_string(),
        investor_harvestable_amount: microalgos_to_algos(can_harvest).to_string(),
        investor_harvestable_amount_microalgos: can_harvest.to_string(),
    })
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct LoadInvestmentParJs {
    pub project_id: String,
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
