use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BalanceProvider {
    async fn get(&self, pars: BalanceParJs) -> Result<BalanceResJs, FrError>;
    async fn get_balance_change(
        &self,
        pars: BalanceChangeParJs,
    ) -> Result<BalanceChangeResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceParJs {
    pub address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceResJs {
    pub balance_algos: String,
    pub balance_funds_asset: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceChangeParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceChangeResJs {
    pub change: String,
}
