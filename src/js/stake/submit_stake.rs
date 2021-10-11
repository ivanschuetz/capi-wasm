use crate::dependencies::environment;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::{dependencies::algod, js::common::SignedTxFromJs};
use anyhow::Result;
use make::flows::stake::logic::{submit_stake, StakeSigned};
use make::network_util::wait_for_pending_transaction;
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

    if let Some(app_opt_in) = pars.app_opt_in {
        let central_app_opt_in_tx = signed_js_tx_to_signed_tx1(&app_opt_in)?;
        let res = algod
            .broadcast_signed_transaction(&central_app_opt_in_tx)
            .await?;
        let _ = wait_for_pending_transaction(&algod, &res.tx_id);
    }

    // stake tx group
    let shares_xfer_tx = &pars.txs[0];
    let app_call_tx = &pars.txs[1];
    let res = submit_stake(
        &algod,
        StakeSigned {
            app_call_tx: signed_js_tx_to_signed_tx1(shares_xfer_tx)?,
            shares_xfer_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
        },
    )
    .await?;

    log::debug!("Submit stake res: {:?}", res);

    Ok(SubmitStakeResJs {})
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitStakeParJs {
    pub app_opt_in: Option<SignedTxFromJs>,
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitStakeResJs {}
