use core::dependencies::algod;

use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::service::str_to_algos::microalgos_to_algos;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_balance(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_balance, pars: {:?}", pars);
    to_bridge_res(_bridge_balance(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_balance(pars: BalanceParJs) -> Result<BalanceResJs> {
    let algod = algod();
    let balance = algod
        .account_information(&pars.address.parse().map_err(Error::msg)?)
        .await?
        .amount;
    Ok(BalanceResJs {
        balance: format!("{} Algo", microalgos_to_algos(balance)),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceParJs {
    pub address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceResJs {
    pub balance: String,
}
