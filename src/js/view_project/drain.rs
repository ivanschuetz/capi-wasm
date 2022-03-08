use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use crate::teal::programs;
use anyhow::{Error, Result};
use core::dependencies::{algod, indexer};
use core::flows::create_project::storage::load_project::load_project;
use core::flows::drain::drain::fetch_drain_amount_and_drain;
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
    let algod = algod();
    let indexer = indexer();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs.escrows, &capi_deps)
        .await?
        .project;

    let to_sign = fetch_drain_amount_and_drain(
        &algod,
        &pars.drainer_address.parse().map_err(Error::msg)?,
        project.central_app_id,
        funds_asset_specs().id,
        &capi_deps,
        &project.customer_escrow,
        &project.central_escrow,
    )
    .await?;

    Ok(DrainResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.app_call_tx, to_sign.capi_app_call_tx])?,
        pt: SubmitDrainPassthroughParJs {
            drain_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.drain_tx)?,
            capi_share_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.capi_share_tx)?,
            project_id: project_id.to_string(),
        },
    })
}

// TODO this can be optimized passing the already loaded project from JS
// to not load the project again
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
