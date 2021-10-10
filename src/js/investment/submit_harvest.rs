use crate::dependencies::environment;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::{dependencies::algod, js::common::SignedTxFromJs};
use anyhow::Result;
use make::flows::harvest::logic::{submit_harvest, HarvestSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_harvest(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_harvest, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_harvest(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_harvest(pars: SubmitHarvestParJs) -> Result<SubmitHarvestResJs> {
    let algod = algod(&environment());

    let app_call_tx = &pars.txs[0];
    let pay_fee_tx = &pars.txs[1];

    let res = submit_harvest(
        &algod,
        &HarvestSigned {
            harvest_tx: rmp_serde::from_slice(&pars.pt.harvest_tx_msg_pack)?,
            app_call_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
            pay_fee_tx: signed_js_tx_to_signed_tx1(pay_fee_tx)?,
        },
    )
    .await?;

    log::debug!("Submit harvest res: {:?}", res);

    Ok(SubmitHarvestResJs {})
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitHarvestParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitHarvestPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitHarvestPassthroughParJs {
    pub harvest_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitHarvestResJs {}
