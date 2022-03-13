use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use anyhow::Result;
use core::{
    dependencies::algod,
    flows::pay_dao::pay_dao::{submit_pay_dao, PayDaoSigned},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_pay_dao, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_pay_dao(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_pay_dao(pars: SubmitPayDaoParJs) -> Result<SubmitLockResJs> {
    let algod = algod();

    let res = submit_pay_dao(
        &algod,
        PayDaoSigned {
            tx: signed_js_tx_to_signed_tx1(&pars.tx)?,
        },
    )
    .await?;

    log::debug!("Submit lock res: {:?}", res);

    Ok(SubmitLockResJs {})
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitPayDaoParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitLockResJs {}
