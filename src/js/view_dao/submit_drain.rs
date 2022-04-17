use crate::dependencies::{api, capi_deps};
use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::service::str_to_algos::microalgos_to_algos;
use anyhow::Result;
use core::dependencies::algod;
use core::flows::create_dao::storage::load_dao::load_dao;
use core::flows::drain::drain::{submit_drain_customer_escrow, DrainCustomerEscrowSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_drain, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_drain(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_drain(pars: SubmitDrainParJs) -> Result<SubmitDrainResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    let app_call_tx = &pars.txs[0];

    let res = submit_drain_customer_escrow(
        &algod,
        &DrainCustomerEscrowSigned {
            drain_tx: rmp_serde::from_slice(&pars.pt.drain_tx_msg_pack)?,
            capi_share_tx: rmp_serde::from_slice(&pars.pt.capi_share_tx_msg_pack)?,
            capi_app_call_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
            app_call_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
        },
    )
    .await?;

    log::debug!("Submit drain res: {:?}", res);

    // TODO pass the dao from drain request, no need to fetch again here?

    let dao = load_dao(&algod, pars.pt.dao_id.parse()?, &api, &capi_deps).await?;

    // TODO (low prio) Consider just recalculating instead of new fetch

    let customer_escrow_balance = algod
        .account_information(dao.customer_escrow.address())
        .await?
        .amount;

    let app_balance = algod.account_information(&dao.app_address()).await?.amount;

    Ok(SubmitDrainResJs {
        new_customer_escrow_balance: microalgos_to_algos(customer_escrow_balance).to_string(),
        new_app_balance: microalgos_to_algos(app_balance).to_string(),
    })
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitDrainParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitDrainPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitDrainPassthroughParJs {
    pub drain_tx_msg_pack: Vec<u8>,
    pub capi_share_tx_msg_pack: Vec<u8>,
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitDrainResJs {
    pub new_customer_escrow_balance: String,
    pub new_app_balance: String,
}
