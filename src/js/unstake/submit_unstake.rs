use crate::dependencies::environment;
use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, signed_js_txs_to_signed_tx1, to_bridge_res,
};
use crate::service::constants::WITHDRAWAL_SLOT_COUNT;
use crate::{dependencies::algod, js::common::SignedTxFromJs};
use anyhow::{anyhow, Result};
use make::flows::unstake::logic::{submit_unstake, UnstakeSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_unstake(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_unstake, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_unstake(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_unstake(pars: SubmitUnstakeParJs) -> Result<SubmitUnstakeResJs> {
    let algod = algod(&environment());

    if pars.txs.len() != 2 + WITHDRAWAL_SLOT_COUNT as usize {
        return Err(anyhow!("Invalid unstake txs count: {}", pars.txs.len()));
    }
    let app_call_tx = &pars.txs[0];
    let pay_fee_tx = &pars.txs[1];
    let slots_app_calls_txs = &pars.txs[2..(2 + WITHDRAWAL_SLOT_COUNT as usize)];

    let res = submit_unstake(
        &algod,
        UnstakeSigned {
            central_app_optout_tx: signed_js_tx_to_signed_tx1(app_call_tx)?,
            slot_optout_txs: signed_js_txs_to_signed_tx1(slots_app_calls_txs)?,
            shares_xfer_tx_signed: rmp_serde::from_slice(&pars.pt.shares_xfer_tx_msg_pack)?,
            pay_shares_xfer_fee_tx: signed_js_tx_to_signed_tx1(pay_fee_tx)?,
        },
    )
    .await?;

    log::debug!("Submit unstake res: {:?}", res);

    Ok(SubmitUnstakeResJs {})
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUnstakeParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitUnstakePassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitUnstakePassthroughParJs {
    pub shares_xfer_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUnstakeResJs {}
