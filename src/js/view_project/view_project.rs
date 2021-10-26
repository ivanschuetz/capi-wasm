use crate::dependencies::{algod, api, environment};
use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::service::available_funds::available_funds;
use crate::service::load_project_view_data::asset_supply;
use crate::service::str_to_algos::microalgos_to_algos;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use make::api::json_workaround::ProjectForUsersJson;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_view_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_view_project, pars: {:?}", pars);
    to_bridge_res(_bridge_view_project(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_view_project(pars: ViewProjectParJs) -> Result<ViewProjectResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project_user_view(&pars.project_id).await?;

    let shares_supply = asset_supply(&algod, project.shares_asset_id).await?;

    let shares_available = algod
        .account_information(&project.invest_escrow_address)
        .await?
        .assets
        .iter()
        .find(|a| a.asset_id == project.shares_asset_id)
        .ok_or({
            anyhow!("Invalid app state: Investor escrow doesn't have shares asset, Please contact support.")})?.amount;

    // TODO investor count: get all holders of asset (indexer?)

    let customer_payment_deeplink =
        LinkableTransactionBuilder::payment(project.customer_escrow_address, MicroAlgos(0))
            .build()
            .as_url();

    let available_funds = available_funds(&algod, &project).await?;

    let investos_share_formatted = format!("{} %", project.investors_share.to_string());

    Ok(ViewProjectResJs {
        project: project.into(),
        shares_supply: shares_supply.to_string(),
        shares_available: shares_available.to_string(),
        investors_share: investos_share_formatted,
        available_funds: microalgos_to_algos(available_funds).to_string(),
        customer_payment_deeplink: customer_payment_deeplink.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct ViewProjectParJs {
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewProjectResJs {
    pub project: ProjectForUsersJson,
    pub shares_supply: String,
    pub shares_available: String,
    pub investors_share: String,
    pub available_funds: String,
    pub customer_payment_deeplink: String,
}
