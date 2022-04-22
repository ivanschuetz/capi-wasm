use super::req_delay;
use crate::provider::holders_count_provider::{
    HoldersCountParJs, HoldersCountProvider, HoldersCountResJs,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct HoldersCountProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HoldersCountProvider for HoldersCountProviderMock {
    async fn get(&self, _: HoldersCountParJs) -> Result<HoldersCountResJs> {
        req_delay().await;

        Ok(HoldersCountResJs {
            count: "2315".to_string(),
        })
    }
}
