use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HoldersCountProvider {
    async fn get(&self, pars: HoldersCountParJs) -> Result<HoldersCountResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct HoldersCountParJs {
    pub asset_id: String,
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HoldersCountResJs {
    pub count: String,
}
