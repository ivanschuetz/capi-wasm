use anyhow::Result;
use async_trait::async_trait;

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DescriptionProvider {
    async fn get(&self, id: String) -> Result<String, FrError>;
}
