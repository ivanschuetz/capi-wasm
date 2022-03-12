use crate::dependencies::capi_deps;
use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::service::drain_if_needed::submit_drain;
use crate::teal::programs;
use anyhow::{anyhow, Error, Result};
use core::dependencies::{algod, indexer};
use core::diagnostics::log_harvest_diagnostics;
use core::flows::create_project::storage::load_project::load_project;
use core::flows::harvest::harvest::{submit_harvest, HarvestSigned};
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
    let algod = algod();
    let indexer = indexer();
    let capi_deps = capi_deps()?;
    let programs = programs();

    // 1 tx if only harvest, 3 if harvest + 2 drain
    if pars.txs.len() != 1 && pars.txs.len() != 3 {
        return Err(anyhow!("Unexpected harvest txs length: {}", pars.txs.len()));
    }
    // sanity check
    if pars.txs.len() == 1 && pars.pt.maybe_drain_tx_msg_pack.is_some() {
        return Err(anyhow!(
            "Invalid state: 1 tx with a passthrough draining tx",
        ));
    }

    if pars.txs.len() == 3 {
        let drain_tx = &pars.pt.maybe_drain_tx_msg_pack
            .ok_or_else(|| anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed drain tx"))?;

        let capi_share_tx = &pars.pt.maybe_capi_share_tx_msg_pack
            .ok_or_else(|| anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed capi share tx"))?;

        submit_drain(&algod, drain_tx, &pars.txs[1], &capi_share_tx, &pars.txs[2]).await?;
    }

    let app_call_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;

    ///////////////////////////
    let project = load_project(
        &algod,
        &indexer,
        &pars.project_id_for_diagnostics.parse()?,
        &programs.escrows,
        &capi_deps,
    )
    .await?
    .project;

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
    pub maybe_capi_share_tx_msg_pack: Option<Vec<u8>>,
    pub harvest_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitHarvestResJs {}
