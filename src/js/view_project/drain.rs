use crate::dependencies::{algod, api, environment};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use anyhow::{Error, Result};
use core::flows::drain::drain::drain_customer_escrow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

use super::submit_drain::SubmitDrainPassthroughParJs;

#[wasm_bindgen]
pub async fn bridge_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_drain, pars: {:?}", pars);
    to_bridge_res(_bridge_drain(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_drain(pars: DrainParJs) -> Result<DrainResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let to_sign = drain_customer_escrow(
        &algod,
        &pars.drainer_address.parse().map_err(Error::msg)?,
        project.central_app_id,
        &project.customer_escrow,
        &project.central_escrow,
    )
    .await?;

    log::debug!("Amount to drain: {:?}", to_sign.amount_to_drain);

    Ok(DrainResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.app_call_tx, to_sign.pay_fee_tx])?,
        pt: SubmitDrainPassthroughParJs {
            drain_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.drain_tx)?,
            project_id: pars.project_id,
        },
    })
}

// TODO this can be optimized passing the already loaded project from JS
// to not call the api again to load the project
// (we'd have to use the complete project instance - drain needs lsig)
#[derive(Debug, Clone, Deserialize)]
pub struct DrainParJs {
    pub project_id: String,
    pub drainer_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DrainResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitDrainPassthroughParJs,
}
