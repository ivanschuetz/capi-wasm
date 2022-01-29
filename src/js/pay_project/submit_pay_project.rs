use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use anyhow::Result;
use core::{
    dependencies::algod,
    flows::pay_project::pay_project::{submit_pay_project, PayProjectSigned},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_pay_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_pay_project, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_pay_project(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_pay_project(pars: SubmitPayProjectParJs) -> Result<SubmitStakeResJs> {
    let algod = algod();

    let res = submit_pay_project(
        &algod,
        PayProjectSigned {
            tx: signed_js_tx_to_signed_tx1(&pars.tx)?,
        },
    )
    .await?;

    log::debug!("Submit stake res: {:?}", res);

    Ok(SubmitStakeResJs {})
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitPayProjectParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitStakeResJs {}
