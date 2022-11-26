use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DividendsProvider {
    async fn get(&self, pars: DividendsParJs) -> Result<String, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct DividendsParJs {
    pub investor_address: String,
    pub dao_id: String,
}
