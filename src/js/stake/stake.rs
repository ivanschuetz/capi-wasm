// TODO probably file can be deleted - we don't need bridge only to get votes? if not check repeated code with get_votes_percentage in load_requests

use crate::dependencies::environment;
use crate::service::account_state::asset_holdings;
use crate::{
    dependencies::{algod, api},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
};
use anyhow::{Error, Result};
use make::flows::stake::logic::stake;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_stake(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_stake, pars: {:?}", pars);
    to_bridge_res(_bridge_stake(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_stake(pars: StakeParJs) -> Result<StakeResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let investor_shares_count =
        asset_holdings(&algod, &investor_address, project.shares_asset_id).await?;

    let to_sign = stake(
        &algod,
        investor_address,
        investor_shares_count,
        project.shares_asset_id,
        project.central_app_id,
        &project.withdrawal_slot_ids,
        &project.staking_escrow,
    )
    .await?;

    let mut to_sign_txs = vec![to_sign.central_app_call_setup_tx, to_sign.shares_xfer_tx];
    to_sign_txs.extend(to_sign.slot_setup_app_calls_txs);

    Ok(StakeResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs)?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct StakeParJs {
    pub project_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StakeResJs {
    pub to_sign: Vec<Value>,
}
