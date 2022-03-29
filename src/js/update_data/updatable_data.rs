use crate::js::common::{parse_bridge_pars, to_bridge_res};
use anyhow::{Error, Result};
use core::dependencies::algod;
use core::flows::create_dao::storage::load_dao::DaoId;
use core::state::dao_app_state::dao_global_state;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// To pre fill the form to update data
#[wasm_bindgen]
pub async fn bridge_updatable_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_updatable_data, pars: {:?}", pars);

    to_bridge_res(_bridge_updatable_data(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_updatable_data(pars: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
    let algod = algod();

    let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

    let app_state = dao_global_state(&algod, dao_id.0).await?;

    Ok(UpdatableDataResJs {
        owner: app_state.owner.to_string(),
        central_escrow: app_state.central_escrow.address.to_string(),
        customer_escrow: app_state.customer_escrow.address.to_string(),
        investing_escrow: app_state.investing_escrow.address.to_string(),
        locking_escrow: app_state.locking_escrow.address.to_string(),
        central_escrow_version: app_state.central_escrow.version.0.to_string(),
        customer_escrow_version: app_state.customer_escrow.version.0.to_string(),
        investing_escrow_version: app_state.investing_escrow.version.0.to_string(),
        locking_escrow_version: app_state.locking_escrow.version.0.to_string(),
        project_name: app_state.project_name,
        project_desc: app_state.project_desc,
        share_price: app_state.share_price.to_string(),
        logo_url: app_state.logo_url,
        social_media_url: app_state.social_media_url,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatableDataParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatableDataResJs {
    pub owner: String,

    pub central_escrow: String,
    pub customer_escrow: String,
    pub investing_escrow: String,
    pub locking_escrow: String,

    pub central_escrow_version: String,
    pub customer_escrow_version: String,
    pub investing_escrow_version: String,
    pub locking_escrow_version: String,

    pub project_name: String,
    pub project_desc: String,
    pub share_price: String,

    pub logo_url: String,
    pub social_media_url: String,
}
