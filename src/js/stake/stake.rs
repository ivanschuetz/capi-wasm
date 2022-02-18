use crate::{
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
    teal::programs,
};
use anyhow::{Error, Result};
use core::{
    dependencies::algod,
    flows::{create_project::storage::load_project::load_project, stake::stake::stake},
};
use core::{dependencies::indexer, flows::create_project::share_amount::ShareAmount};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_stake(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_stake, pars: {:?}", pars);
    to_bridge_res(_bridge_stake(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_stake(pars: StakeParJs) -> Result<StakeResJs> {
    let algod = algod();
    let indexer = indexer();

    let share_amount = ShareAmount(pars.share_count.parse()?);

    let stored_project = load_project(
        &algod,
        &indexer,
        &pars.project_id.parse()?,
        &programs().escrows,
    )
    .await?;

    let project = stored_project.project;

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let to_sign = stake(
        &algod,
        investor_address,
        share_amount,
        project.shares_asset_id,
        project.central_app_id,
        &project.staking_escrow,
        &stored_project.id,
    )
    .await?;

    let to_sign_txs = vec![to_sign.central_app_call_setup_tx, to_sign.shares_xfer_tx];

    Ok(StakeResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs)?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct StakeParJs {
    pub project_id: String,
    pub investor_address: String,
    pub share_count: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StakeResJs {
    pub to_sign: Vec<Value>,
}
