use crate::dependencies::capi_deps;
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_tx1};
use crate::service::constants::PRECISION;
use crate::teal::programs;
use anyhow::{Error, Result};
use core::dependencies::algod;
use core::flows::create_dao::setup::create_app::render_central_app;
use core::flows::create_dao::storage::load_dao::load_dao;
use core::flows::update_app::update::update;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_update_app_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_update_app_txs, pars: {:?}", pars);
    to_bridge_res(_bridge_update_app_txs(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_update_app_txs(pars: UpdateDaoAppParJs) -> Result<UpdateDaoAppResJs> {
    let algod = algod();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let dao_id = pars.dao_id.parse().map_err(Error::msg)?;
    let owner = pars.owner.parse().map_err(Error::msg)?;
    // TODO versioning
    // flow:
    // 1) user selects version number on UI (needs also a new service to check for and display new versions)
    // 2) fetch template for that version number (e.g. using strings like currently or download from somewhere)
    // 3) call redering function for that version (should be implemented in core)
    // Note that the current core "render_central_app" function is essentially for version 1.
    // Side note: consider adding version as a comment in TEAL and check in the render functions (for a bit more security re: passing the correct template versions to the rendering functions)
    let _version: u64 = pars.version.parse().map_err(Error::msg)?;

    // TODO optimize: instead of calling load_dao, fetch app state and asset infos (don't e.g. compile and render the escrows, which is not needed here)
    let dao = load_dao(&algod, dao_id, &programs.escrows, &capi_deps).await?;

    // TODO versioning
    // since there's no versioning, we just render again with v1
    let app_source = render_central_app(
        &programs.central_app_approval,
        &owner,
        dao.specs.shares.supply,
        PRECISION,
        dao.specs.investors_part(),
        &capi_deps.escrow,
        capi_deps.app_id,
        capi_deps.escrow_percentage,
        dao.specs.share_price,
    )?;

    let to_sign = update(
        &algod,
        &owner,
        dao_id.0,
        app_source,
        programs.central_app_clear,
    )
    .await?;

    Ok(UpdateDaoAppResJs {
        to_sign: to_my_algo_tx1(&to_sign.update).map_err(Error::msg)?,
    })
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDaoAppParJs {
    pub dao_id: String,
    pub owner: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDaoAppResJs {
    pub to_sign: Value,
}
