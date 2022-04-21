use crate::js::common::{parse_bridge_pars, to_bridge_res};
use algonaut::core::to_app_address;
use anyhow::Result;
use base::{dependencies::indexer, queries::shares_distribution::holders_count};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_holders_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_holders_count, pars: {:?}", pars);
    to_bridge_res(_bridge_holders_count(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_holders_count(pars: HoldersCountParJs) -> Result<HoldersCountResJs> {
    let indexer = indexer();

    let asset_id = pars.asset_id.parse()?;

    let app_id = pars.app_id.parse()?;

    let app_address = to_app_address(app_id);

    Ok(HoldersCountResJs {
        count: holders_count(&indexer, asset_id, &app_address)
            .await?
            .to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct HoldersCountParJs {
    pub asset_id: String,
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HoldersCountResJs {
    pub count: String,
}
