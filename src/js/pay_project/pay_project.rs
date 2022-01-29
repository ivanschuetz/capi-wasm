use crate::{
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_tx1},
    service::str_to_algos::validate_algos_input,
};
use anyhow::{Error, Result};
use core::{dependencies::algod, flows::pay_project::pay_project::pay_project};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_pay_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_pay_project, pars: {:?}", pars);
    to_bridge_res(_bridge_pay_project(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_pay_project(pars: PayProjectParJs) -> Result<PayProjectResJs> {
    let algod = algod();

    let customer_address = pars.customer_address.parse().map_err(Error::msg)?;
    let customer_escrow_address = pars.customer_escrow_address.parse().map_err(Error::msg)?;
    let amount = validate_algos_input(&pars.amount)?;

    let to_sign = pay_project(&algod, &customer_address, &customer_escrow_address, amount).await?;

    Ok(PayProjectResJs {
        to_sign: to_my_algo_tx1(&to_sign.tx)?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct PayProjectParJs {
    pub customer_address: String,
    pub customer_escrow_address: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PayProjectResJs {
    pub to_sign: Value,
}
