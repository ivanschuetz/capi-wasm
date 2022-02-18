use crate::js::common::{parse_bridge_pars, to_bridge_res};
use anyhow::{Error, Result};
use core::{dependencies::indexer, queries::shares_distribution::holders_count};
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
    let investing_escrow = pars.investing_escrow_address.parse().map_err(Error::msg)?;
    let locking_escrow = pars.locking_escrow_address.parse().map_err(Error::msg)?;

    Ok(HoldersCountResJs {
        count: holders_count(&indexer, asset_id, &investing_escrow, &locking_escrow)
            .await?
            .to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct HoldersCountParJs {
    pub asset_id: String,
    pub investing_escrow_address: String,
    pub locking_escrow_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HoldersCountResJs {
    pub count: String,
}
