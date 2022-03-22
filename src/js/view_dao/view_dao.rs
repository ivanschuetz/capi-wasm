use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::model::dao_for_users::dao_to_dao_for_users;
use crate::model::dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData};
use crate::service::available_funds::available_funds;
use crate::service::str_to_algos::base_units_to_display_units;
use crate::teal::programs;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use core::dependencies::algod;
use core::flows::create_dao::storage::load_dao::load_dao;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_view_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_view_dao, pars: {:?}", pars);
    to_bridge_res(_bridge_view_dao(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_view_dao(pars: ViewDaoParJs) -> Result<ViewDaoResJs> {
    let algod = algod();
    let funds_asset_specs = funds_asset_specs();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &programs.escrows, &capi_deps).await?;

    // TODO investor count: get all holders of asset (indexer?)

    let customer_payment_deeplink =
        LinkableTransactionBuilder::payment(*dao.customer_escrow.address(), MicroAlgos(0))
            .build()
            .as_url();

    let available_funds = available_funds(&algod, &dao, funds_asset_specs.id).await?;

    let shares_available = algod
        .account_information(dao.invest_escrow.address())
        .await?
        .assets
        .iter()
        .find(|a| a.asset_id == dao.shares_asset_id)
        .ok_or({
            anyhow!("Invalid app state: Investor escrow doesn't have shares asset, Please contact support.")
        })?.amount;

    let investos_share_formatted = format!("{} %", dao.specs.investors_part());

    let dao_view_data =
        dao_for_users_to_view_data(dao_to_dao_for_users(&dao, &dao_id)?, &funds_asset_specs);

    Ok(ViewDaoResJs {
        dao: dao_view_data,
        // shares_supply: shares_supply.to_string(),
        shares_available: shares_available.to_string(),
        investors_share: investos_share_formatted,
        available_funds: base_units_to_display_units(available_funds, &funds_asset_specs)
            .to_string(),
        customer_payment_deeplink: customer_payment_deeplink.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct ViewDaoParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewDaoResJs {
    pub dao: DaoForUsersViewData,
    // pub shares_supply: String,
    pub shares_available: String,
    pub investors_share: String,
    pub available_funds: String,
    pub customer_payment_deeplink: String,
}
