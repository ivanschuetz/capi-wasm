use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WithdrawalHistoryProvider {
    async fn get(&self, pars: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadWithdrawalParJs {
    pub dao_id: String,
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
