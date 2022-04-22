use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait OptinToAppProvider {
    async fn txs(&self, pars: OptInToAppParJs) -> Result<OptInToAppResJs>;
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct OptInToAppParJs {
    pub app_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptInToAppResJs {
    pub to_sign: Option<Vec<Value>>,
}
