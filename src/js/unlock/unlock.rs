use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use crate::js::unlock::submit_unlock::SubmitUnlockPassthroughParJs;
use crate::teal::programs;
use anyhow::{Error, Result};
use core::dependencies::{algod, indexer};
use core::flows::create_project::storage::load_project::load_project;
use core::flows::unlock::unlock::unlock;
use core::state::central_app_state::central_investor_state;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_unlock, pars: {:?}", pars);
    to_bridge_res(_bridge_unlock(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_unlock(pars: UnlockParJs) -> Result<UnlockResJs> {
    let algod = algod();
    let indexer = indexer();

    let project = load_project(
        &algod,
        &indexer,
        &pars.project_id.parse()?,
        &programs().escrows,
    )
    .await?
    .project;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let investor_state =
        central_investor_state(&algod, &investor_address, project.central_app_id).await?;

    log::debug!("Unlocking shares: {:?}", investor_state.shares);

    let to_sign = unlock(
        &algod,
        investor_address,
        investor_state.shares,
        project.shares_asset_id,
        project.central_app_id,
        &project.locking_escrow,
    )
    .await?;

    let to_sign_txs = vec![to_sign.central_app_optout_tx];

    Ok(UnlockResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs)?,
        pt: SubmitUnlockPassthroughParJs {
            shares_xfer_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.shares_xfer_tx)?,
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnlockParJs {
    pub project_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnlockResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitUnlockPassthroughParJs,
}
