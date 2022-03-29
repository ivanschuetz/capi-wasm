use crate::dependencies::api;
use crate::js::common::{parse_bridge_pars, to_bridge_res};
use anyhow::{Error, Result};
use core::api::api::Api;
use core::api::version::Version;
use core::dependencies::algod;
use core::flows::create_dao::storage::load_dao::DaoId;
use core::state::dao_app_state::dao_global_state;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_check_for_updates(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_check_for_updates, pars: {:?}", pars);
    to_bridge_res(_bridge_check_for_updates(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_check_for_updates(pars: CheckForUpdatesParJs) -> Result<CheckForUpdatesResJs> {
    let algod = algod();
    let api = api();

    let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

    let state = dao_global_state(&algod, dao_id.0).await?;
    let last_versions = api.last_versions();

    let update_data = if last_versions.app_approval.0 > state.app_approval_version.0
        || last_versions.app_clear.0 > state.app_clear_version.0
    {
        Some(UpdateData {
            new_approval_version: last_versions.app_approval,
            new_clear_version: last_versions.app_clear,
        })
    } else {
        None
    };

    Ok(CheckForUpdatesResJs {
        current_approval_version: state.app_approval_version.0.to_string(),
        current_clear_version: state.app_clear_version.0.to_string(),
        update_data: update_data.map(|d| update_data_to_js(d)),
    })
}

#[derive(Debug)]
struct UpdateData {
    new_approval_version: Version,
    new_clear_version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckForUpdatesParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckForUpdatesResJs {
    pub current_approval_version: String,
    pub current_clear_version: String,

    pub update_data: Option<UpdateDataJs>, // set if there's an update
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDataJs {
    pub new_approval_version: String,
    pub new_clear_version: String,
}

fn update_data_to_js(data: UpdateData) -> UpdateDataJs {
    UpdateDataJs {
        new_approval_version: data.new_approval_version.0.to_string(),
        new_clear_version: data.new_clear_version.0.to_string(),
    }
}
