use crate::dependencies::{api, capi_deps};
use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::service::drain_if_needed::submit_drain;
use anyhow::{anyhow, Error, Result};
use core::dependencies::algod;
use core::diagnostics::log_claim_diagnostics;
use core::flows::claim::claim::{submit_claim, ClaimSigned};
use core::flows::create_dao::storage::load_dao::load_dao;
use core::network_util::wait_for_pending_transaction;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_claim, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_claim(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_claim(pars: SubmitClaimParJs) -> Result<SubmitClaimResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    // 1 tx if only claim, 3 if claim + 2 drain
    if pars.txs.len() != 1 && pars.txs.len() != 3 {
        return Err(anyhow!("Unexpected claim txs length: {}", pars.txs.len()));
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
    let dao = load_dao(
        &algod,
        pars.dao_id_for_diagnostics.parse()?,
        &api,
        &capi_deps,
    )
    .await?;

    log_claim_diagnostics(
        &algod,
        &pars
            .investor_address_for_diagnostics
            .parse()
            .map_err(Error::msg)?,
        &dao,
    )
    .await?;
    ///////////////////////////

    let claim_tx_id = submit_claim(
        &algod,
        &ClaimSigned {
            app_call_tx_signed: app_call_tx,
        },
    )
    .await?;

    log::warn!("Submit claim tx id: {:?}", claim_tx_id);
    wait_for_pending_transaction(&algod, &claim_tx_id).await?;

    Ok(SubmitClaimResJs {})
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitClaimParJs {
    pub investor_address_for_diagnostics: String,
    pub dao_id_for_diagnostics: String,

    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitClaimPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitClaimPassthroughParJs {
    // set if a drain tx is necessary
    pub maybe_drain_tx_msg_pack: Option<Vec<u8>>,
    pub maybe_capi_share_tx_msg_pack: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitClaimResJs {}
