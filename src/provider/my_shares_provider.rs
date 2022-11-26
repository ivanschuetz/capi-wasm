use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait MySharesProvider {
    async fn get(&self, pars: MySharesParJs) -> Result<MySharesResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct MySharesParJs {
    pub dao_id: String,
    pub my_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MySharesResJs {
    pub total: String,
    pub free: String,
    pub locked: String,
}
