use crate::{
    error::FrError,
    provider::my_shares_provider::{MySharesParJs, MySharesProvider, MySharesResJs},
};
use anyhow::Result;
use async_trait::async_trait;

use super::req_delay;

pub struct MySharesProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl MySharesProvider for MySharesProviderMock {
    async fn get(&self, _: MySharesParJs) -> Result<MySharesResJs, FrError> {
        req_delay().await;

        Ok(MySharesResJs {
            total: "2000".to_owned(),
            free: "500".to_owned(),
            locked: "1500".to_owned(),
        })
    }
}
