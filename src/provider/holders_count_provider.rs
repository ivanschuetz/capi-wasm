use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HoldersCountProvider {
    async fn get(&self, pars: HoldersCountParJs) -> Result<HoldersCountResJs, FrError>;

    /// returns "up"/"down"/"eq" if holders went up/down/staid the same, and "unknown" if nothing to compare with
    /// note that here we use local storage (this is currently inconsistent with other places with "change"),
    /// thus "unknown" if there's no older entry to compare with.
    async fn change(&self, pars: HoldersChangeParJs) -> Result<HoldersChangeResJs, FrError>;
}

// TODO use dao_id (convention?), in HoldersChangeParJs too

#[derive(Debug, Clone, Deserialize)]
pub struct HoldersCountParJs {
    pub asset_id: String,
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HoldersCountResJs {
    pub count: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HoldersChangeParJs {
    pub asset_id: String,
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HoldersChangeResJs {
    pub change: String,
}
