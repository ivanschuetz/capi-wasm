use crate::{
    dependencies::capi_deps,
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
    teal::programs,
};
use anyhow::{Error, Result};
use core::flows::create_dao::share_amount::ShareAmount;
use core::{
    dependencies::algod,
    flows::{create_dao::storage::load_dao::load_dao, lock::lock::lock},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_lock, pars: {:?}", pars);
    to_bridge_res(_bridge_lock(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_lock(pars: LockParJs) -> Result<LockResJs> {
    let algod = algod();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let share_amount = ShareAmount::new(pars.share_count.parse()?);

    let dao = load_dao(&algod, pars.dao_id.parse()?, &programs.escrows, &capi_deps).await?;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let to_sign = lock(
        &algod,
        investor_address,
        share_amount,
        dao.shares_asset_id,
        dao.app_id,
        &dao.locking_escrow.address(),
        dao.id(),
    )
    .await?;

    let to_sign_txs = vec![to_sign.central_app_call_setup_tx, to_sign.shares_xfer_tx];

    Ok(LockResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs)?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct LockParJs {
    pub dao_id: String,
    pub investor_address: String,
    pub share_count: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LockResJs {
    pub to_sign: Vec<Value>,
}
