use crate::{
    error::FrError,
    provider::lock_provider::{
        LockParJs, LockProvider, LockResJs, SubmitLockParJs, SubmitLockResJs,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

use super::{mock_to_sign, req_delay};

pub struct LockProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl LockProvider for LockProviderMock {
    async fn txs(&self, pars: LockParJs) -> Result<LockResJs, FrError> {
        let algod = algod();

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(LockResJs {
            to_sign: mock_to_sign(&algod, &investor_address).await?,
        })
    }

    async fn submit(&self, _: SubmitLockParJs) -> Result<SubmitLockResJs> {
        req_delay().await;

        Ok(SubmitLockResJs {})
    }
}
