use crate::provider::lock_provider::{
    LockParJs, LockProvider, LockResJs, SubmitLockParJs, SubmitLockResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::algod;

use super::{mock_js_txs, req_delay};

pub struct LockProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl LockProvider for LockProviderMock {
    async fn txs(&self, pars: LockParJs) -> Result<LockResJs> {
        let algod = algod();

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(LockResJs {
            to_sign: mock_js_txs(&algod, &investor_address).await?,
        })
    }

    async fn submit(&self, _: SubmitLockParJs) -> Result<SubmitLockResJs> {
        req_delay().await;

        Ok(SubmitLockResJs {})
    }
}
