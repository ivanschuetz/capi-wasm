use crate::dependencies::environment;
use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, signed_js_txs_to_signed_tx1, to_bridge_res,
};
use crate::service::constants::WITHDRAWAL_SLOT_COUNT;
use crate::service::invest_or_stake::submit_apps_optins_from_js;
use crate::{dependencies::algod, js::common::SignedTxFromJs};
use anyhow::{anyhow, Result};
use make::flows::stake::logic::{submit_stake, StakeSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_stake(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_stake, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_stake(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_stake(pars: SubmitStakeParJs) -> Result<SubmitStakeResJs> {
    let algod = algod(&environment());

    if let Some(app_opt_ins) = pars.app_opt_ins {
        submit_apps_optins_from_js(&algod, &app_opt_ins).await?;
    }

    // sanity check
    if pars.txs.len() != 2 + WITHDRAWAL_SLOT_COUNT as usize {
        return Err(anyhow!("Invalid app optins count: {}", pars.txs.len()));
    }

    // stake tx group
    let central_app_call_tx = &pars.txs[0];
    let shares_xfer_tx = &pars.txs[1];
    let slots_app_calls_txs = &pars.txs[1..(1 + WITHDRAWAL_SLOT_COUNT as usize)];

    let res = submit_stake(
        &algod,
        StakeSigned {
            central_app_call_setup_tx: signed_js_tx_to_signed_tx1(shares_xfer_tx)?,
            slot_setup_app_calls_txs: signed_js_txs_to_signed_tx1(slots_app_calls_txs)?,
            shares_xfer_tx_signed: signed_js_tx_to_signed_tx1(central_app_call_tx)?,
        },
    )
    .await?;

    log::debug!("Submit stake res: {:?}", res);

    Ok(SubmitStakeResJs {})
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitStakeParJs {
    // Set if user isn't opted in yet (follows bridge_opt_in_to_apps_if_needed)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitStakeResJs {}
