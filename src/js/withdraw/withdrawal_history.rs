use crate::js::common::{parse_bridge_pars, to_bridge_res};
use algonaut::{core::Address, indexer::v2::Indexer};
use anyhow::{Error, Result};
use core::{dependencies::indexer, queries::withdrawals::withdrawals};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::withdrawal_view_data;

#[wasm_bindgen]
pub async fn bridge_load_withdrawals(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_withdrawals, pars: {:?}", pars);
    to_bridge_res(_bridge_load_withdrawals(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_withdrawals(pars: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs> {
    let indexer = indexer();

    let creator = pars.creator_address.parse().map_err(Error::msg)?;

    let entries = load_withdrawals(&indexer, &pars.project_uuid, &creator).await?;

    Ok(LoadWithdrawalResJs { entries })
}

pub async fn load_withdrawals(
    indexer: &Indexer,
    project_uuid: &str,
    creator: &Address,
) -> Result<Vec<WithdrawalViewData>> {
    let entries = withdrawals(indexer, creator, &project_uuid.parse()?).await?;
    let mut reqs_view_data = vec![];
    for entry in entries {
        reqs_view_data.push(withdrawal_view_data(
            entry.amount,
            entry.description,
            entry.date.to_rfc2822(),
            entry.tx_id,
        ));
    }
    Ok(reqs_view_data)
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadWithdrawalParJs {
    pub project_uuid: String,
    pub creator_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadWithdrawalResJs {
    pub entries: Vec<WithdrawalViewData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WithdrawalViewData {
    pub amount: String,
    pub description: String,
    pub date: String,

    pub tx_id: String,
    pub tx_link: String,

    /// passthrough model data
    pub amount_not_formatted: String,
}
