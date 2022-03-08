use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::model::project_for_users::project_to_project_for_users;
use crate::model::project_for_users_view_data::{
    project_for_users_to_view_data, ProjectForUsersViewData,
};
use crate::service::available_funds::available_funds;
use crate::service::str_to_algos::base_units_to_display_units;
use crate::teal::programs;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use core::dependencies::{algod, indexer};
use core::flows::create_project::storage::load_project::load_project;
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
    let funds_asset_specs = funds_asset_specs();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs.escrows, &capi_deps)
        .await?
        .project;

    // TODO investor count: get all holders of asset (indexer?)

    let customer_payment_deeplink =
        LinkableTransactionBuilder::payment(*project.customer_escrow.address(), MicroAlgos(0))
            .build()
            .as_url();

    let available_funds = available_funds(&algod, &project, funds_asset_specs.id).await?;

    let shares_available = algod
        .account_information(project.invest_escrow.address())
        .await?
        .assets
        .iter()
        .find(|a| a.asset_id == project.shares_asset_id)
        .ok_or({
            anyhow!("Invalid app state: Investor escrow doesn't have shares asset, Please contact support.")
        })?.amount;

    let investos_share_formatted = format!("{} %", project.specs.investors_part());

    let project_view_data = project_for_users_to_view_data(
        project_to_project_for_users(&project, &project_id)?,
        &funds_asset_specs,
    );

    Ok(ViewProjectResJs {
        project: project_view_data,
        // shares_supply: shares_supply.to_string(),
        shares_available: shares_available.to_string(),
        investors_share: investos_share_formatted,
        available_funds: base_units_to_display_units(available_funds, &funds_asset_specs)
            .to_string(),
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
