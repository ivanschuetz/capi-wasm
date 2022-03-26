use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use anyhow::Result;
use core::dependencies::algod;
use core::flows::update_app::update::{submit_update, UpdateAppSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_update_app(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_update_app, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_update_app(parse_bridge_pars(pars)?).await)
}

async fn _bridge_submit_update_app(pars: SubmitUpdateAppParJs) -> Result<SubmitUpdateAppResJs> {
    let algod = algod();

    let submit_update_res = submit_update(
        &algod,
        UpdateAppSigned {
            update: signed_js_tx_to_signed_tx1(&pars.tx)?,
        },
    )
    .await?;

    log::debug!("Submit update res: {:?}", submit_update_res);

    Ok(SubmitUpdateAppResJs {})
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUpdateAppParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUpdateAppResJs {}
