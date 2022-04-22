use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait FundsActivityProvider {
    async fn get(&self, pars: LoadFundsActivityParJs) -> Result<LoadFundsActivityResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadFundsActivityParJs {
    pub dao_id: String,
    pub owner_address: String,
    pub max_results: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadFundsActivityResJs {
    pub entries: Vec<FundsActivityViewData>,
}

unsafe impl Send for LoadFundsActivityResJs {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FundsActivityViewData {
    pub amount: String,
    pub is_income: String, // false: spending
    pub description: String,
    pub date: String,
    pub tx_id: String,
    pub tx_link: String,
}
