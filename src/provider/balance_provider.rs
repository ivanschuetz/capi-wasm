use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BalanceProvider {
    async fn get(&self, pars: BalanceParJs) -> Result<BalanceResJs>;
    async fn get_balance_change(&self, pars: BalanceChangeParJs) -> Result<BalanceChangeResJs>;
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
    pub customer_escrow: String,
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceChangeResJs {
    pub change: String,
}
