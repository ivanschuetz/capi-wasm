// TODO probably file can be deleted - we don't need bridge only to get votes? if not check repeated code with get_votes_percentage in load_requests

use crate::dependencies::environment;
use crate::js::unstake::submit_unstake::SubmitUnstakePassthroughParJs;
use crate::service::app_state::owned_shares_count;
use crate::{
    dependencies::{algod, api},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
};
use anyhow::{Error, Result};
use core::flows::unstake::logic::unstake;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_unstake(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_unstake, pars: {:?}", pars);
    to_bridge_res(_bridge_unstake(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_unstake(pars: UnstakeParJs) -> Result<UnstakeResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let investor_shares_count =
        owned_shares_count(&algod, &investor_address, project.central_app_id).await?;

    log::debug!("Unstaking shares: {:?}", investor_shares_count);

    let to_sign = unstake(
        &algod,
        investor_address,
        investor_shares_count,
        project.shares_asset_id,
        project.central_app_id,
        &project.withdrawal_slot_ids,
        &project.staking_escrow,
    )
    .await?;

    let mut to_sign_txs = vec![
        to_sign.central_app_optout_tx,
        to_sign.pay_shares_xfer_fee_tx,
    ];
    to_sign_txs.extend(to_sign.slot_optout_txs);

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
