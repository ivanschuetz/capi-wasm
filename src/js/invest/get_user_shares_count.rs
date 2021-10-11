use crate::{
    dependencies::{algod, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
    service::account_state::asset_holdings,
};
use anyhow::{Error, Result};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_get_user_shares_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_get_user_shares_count, pars: {:?}", pars);

    to_bridge_res(_bridge_get_user_shares_count(parse_bridge_pars(pars)?).await)
}

async fn _bridge_get_user_shares_count(pars: GetUserSharesCountParJs) -> Result<u64> {
    let env = &environment();
    let algod = algod(env);

    Ok(asset_holdings(
        &algod,
        &pars.address.parse().map_err(Error::msg)?,
        pars.shares_asset_id.parse()?,
    )
    .await?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserSharesCountParJs {
    pub address: String,
    pub shares_asset_id: String,
}
