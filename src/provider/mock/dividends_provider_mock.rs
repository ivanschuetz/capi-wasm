use super::req_delay;
use crate::{provider::dividends_provider::{DividendsParJs, DividendsProvider}, error::FrError};
use anyhow::Result;
use async_trait::async_trait;

pub struct DividendsProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DividendsProvider for DividendsProviderMock {
    async fn get(&self, _: DividendsParJs) -> Result<String, FrError> {
        req_delay().await;
        Ok("1234".to_owned())
    }
}
