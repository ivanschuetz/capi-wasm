use crate::dependencies::{api, environment};
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::service::drain_if_needed::submit_drain;
use crate::{dependencies::algod, js::common::SignedTxFromJs};
use anyhow::{anyhow, Error, Result};
use core::diagnostics::log_harvest_diagnostics;
use core::flows::harvest::logic::{submit_harvest, HarvestSigned};
use core::network_util::wait_for_pending_transaction;
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

    // 2 txs if only harvest, 4 if withdrawal + drain
    if pars.txs.len() != 2 && pars.txs.len() != 4 {
        return Err(anyhow!("Unexpected harvest txs length: {}", pars.txs.len()));
    }
    // sanity check
    if pars.txs.len() == 2 {
        if pars.pt.maybe_drain_tx_msg_pack.is_some() {
            return Err(anyhow!(
                "Invalid state: 2 txs with a passthrough draining tx",
            ));
        }
    }

    if pars.txs.len() == 4 {
        submit_drain(
            &algod,
            &pars.pt.maybe_drain_tx_msg_pack
                .ok_or(anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed drain tx"))?,
            &pars.txs[2],
            &pars.txs[3],
        )
        .await?;
    }

    let app_call_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
    let pay_fee_tx = signed_js_tx_to_signed_tx1(&pars.txs[1])?;

    ///////////////////////////
    let api = api(&environment());
    let project = api.load_project(&pars.project_id_for_diagnostics).await?;
    log_harvest_diagnostics(
        &algod,
        &pars
            .investor_address_for_diagnostics
            .parse()
            .map_err(Error::msg)?,
        &project,
    )
    .await?;
    ///////////////////////////

    let harvest_tx_id = submit_harvest(
        &algod,
        &HarvestSigned {
            harvest_tx: rmp_serde::from_slice(&pars.pt.harvest_tx_msg_pack)?,
            app_call_tx_signed: app_call_tx,
            pay_fee_tx,
        },
    )
    .await?;

    log::warn!("Submit harvest tx id: {:?}", harvest_tx_id);
    wait_for_pending_transaction(&algod, &harvest_tx_id).await?;

    Ok(SubmitHarvestResJs {})
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitHarvestParJs {
    pub investor_address_for_diagnostics: String,
    pub project_id_for_diagnostics: String,

    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitHarvestPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitHarvestPassthroughParJs {
    // set if a drain tx is necessary
    pub maybe_drain_tx_msg_pack: Option<Vec<u8>>,
    pub harvest_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitHarvestResJs {}
