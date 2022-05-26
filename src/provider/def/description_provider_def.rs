use crate::provider::description_provider::DescriptionProvider;
use anyhow::Result;
use async_trait::async_trait;
use base::{api::image_api::ImageApi, dependencies::image_api};

pub struct DescriptionProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DescriptionProvider for DescriptionProviderDef {
    async fn get(&self, id: String) -> Result<String> {
        let image_api = image_api();
        image_api.get_descr(&id).await
    }
}
