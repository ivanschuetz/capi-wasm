use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait SharesCountProvider {
    async fn get(&self, pars: GetUserSharesCountParJs) -> Result<String>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserSharesCountParJs {
    pub address: String,
    pub shares_asset_id: String,
}
