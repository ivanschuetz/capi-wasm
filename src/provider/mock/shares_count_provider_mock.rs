use super::req_delay;
use crate::{provider::shares_count_provider::{GetUserSharesCountParJs, SharesCountProvider}, error::FrError};
use anyhow::Result;
use async_trait::async_trait;

pub struct SharesCountProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl SharesCountProvider for SharesCountProviderMock {
    async fn get(&self, _: GetUserSharesCountParJs) -> Result<String, FrError> {
        req_delay().await;

        Ok("123".to_owned())
    }
}
