use super::req_delay;
use crate::provider::description_provider::DescriptionProvider;
use anyhow::Result;
use async_trait::async_trait;

pub struct DescriptionProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DescriptionProvider for DescriptionProviderMock {
    async fn get(&self, _id: String) -> Result<String> {
        req_delay().await;

        Ok("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_owned())
    }
}
