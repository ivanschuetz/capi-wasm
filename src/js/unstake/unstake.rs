use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use crate::js::unstake::submit_unstake::SubmitUnstakePassthroughParJs;
use crate::teal::programs;
use anyhow::{Error, Result};
use core::dependencies::{algod, indexer};
use core::flows::create_project::storage::load_project::{load_project, ProjectId};
use core::flows::unstake::unstake::unstake;
use core::state::central_app_state::central_investor_state;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_unstake(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_unstake, pars: {:?}", pars);
    to_bridge_res(_bridge_unstake(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_unstake(pars: UnstakeParJs) -> Result<UnstakeResJs> {
    let algod = algod();
    let indexer = indexer();

    let project = load_project(
        &algod,
        &indexer,
        &ProjectId(pars.project_id),
        &programs().escrows,
    )
    .await?;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let investor_state =
        central_investor_state(&algod, &investor_address, project.central_app_id).await?;

    log::debug!("Unstaking shares: {:?}", investor_state.shares);

    let to_sign = unstake(
        &algod,
        investor_address,
        investor_state.shares,
        project.shares_asset_id,
        project.central_app_id,
        &project.staking_escrow,
    )
    .await?;

    let to_sign_txs = vec![
        to_sign.central_app_optout_tx,
        to_sign.pay_shares_xfer_fee_tx,
    ];

    Ok(UnstakeResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs)?,
        pt: SubmitUnstakePassthroughParJs {
            shares_xfer_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.shares_xfer_tx)?,
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnstakeParJs {
    pub project_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnstakeResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitUnstakePassthroughParJs,
}
