use crate::{error::FrError, provider::description_provider::DescriptionProvider};
use anyhow::Result;
use async_trait::async_trait;
use base::{api::fetcher::Fetcher, dependencies::fetcher};

pub struct DescriptionProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DescriptionProvider for DescriptionProviderDef {
    async fn get(&self, url: String) -> Result<String, FrError> {
        let fetcher = fetcher();
        let bytes = fetcher.get(&url).await?;
        Ok(String::from_utf8(bytes)?)
    }
}
