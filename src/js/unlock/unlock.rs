use crate::dependencies::{api, capi_deps};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use anyhow::{Error, Result};
use base::dependencies::algod;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::unlock::unlock::unlock;
use base::state::dao_app_state::dao_investor_state;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_unlock, pars: {:?}", pars);
    to_bridge_res(_bridge_unlock(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_unlock(pars: UnlockParJs) -> Result<UnlockResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    let dao = load_dao(&algod, pars.dao_id.parse()?, &api, &capi_deps).await?;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let investor_state = dao_investor_state(&algod, &investor_address, dao.app_id).await?;

    log::debug!("Unlocking shares: {:?}", investor_state.shares);

    let to_sign = unlock(&algod, investor_address, dao.app_id, dao.shares_asset_id).await?;

    let to_sign_txs = vec![to_sign.central_app_optout_tx];

    Ok(UnlockResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs)?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnlockParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnlockResJs {
    pub to_sign: Vec<Value>,
}
