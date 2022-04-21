use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AppUpdatesProvider {
    async fn get(&self, pars: CheckForUpdatesParJs) -> Result<CheckForUpdatesResJs>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckForUpdatesParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckForUpdatesResJs {
    pub current_approval_version: String,
    pub current_clear_version: String,

    pub update_data: Option<UpdateDataJs>, // set if there's an update
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDataJs {
    pub new_approval_version: String,
    pub new_clear_version: String,
}
