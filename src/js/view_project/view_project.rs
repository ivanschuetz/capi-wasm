use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::model::project_for_users::project_to_project_for_users;
use crate::model::project_for_users_view_data::ProjectForUsersViewData;
use crate::service::available_funds::available_funds;
use crate::service::str_to_algos::microalgos_to_algos;
use crate::teal::programs;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use core::dependencies::{algod, env, indexer};
use core::flows::create_project::storage::load_project::{load_project, ProjectId};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_view_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_view_project, pars: {:?}", pars);
    to_bridge_res(_bridge_view_project(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_view_project(pars: ViewProjectParJs) -> Result<ViewProjectResJs> {
    let algod = algod();
    let indexer = indexer();
    let env = env();

    let project_id = ProjectId(pars.project_id);

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows).await?;

    // TODO investor count: get all holders of asset (indexer?)

    let customer_payment_deeplink =
        LinkableTransactionBuilder::payment(*project.customer_escrow.address(), MicroAlgos(0))
            .build()
            .as_url();

    let available_funds = available_funds(&algod, &project).await?;

    let shares_available = algod
        .account_information(project.invest_escrow.address())
        .await?
        .assets
        .iter()
        .find(|a| a.asset_id == project.shares_asset_id)
        .ok_or({
            anyhow!("Invalid app state: Investor escrow doesn't have shares asset, Please contact support.")
        })?.amount;

    let investos_share_formatted = format!("{} %", project.specs.investors_share.to_string());

    let project_view_data = project_to_project_for_users(&env, &project, &project_id)?.into();

    Ok(ViewProjectResJs {
        project: project_view_data,
        // shares_supply: shares_supply.to_string(),
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
    pub project: ProjectForUsersViewData,
    // pub shares_supply: String,
    pub shares_available: String,
    pub investors_share: String,
    pub available_funds: String,
    pub customer_payment_deeplink: String,
}
