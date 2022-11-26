use crate::error::FrError;
use crate::provider::mock::req_delay;
use crate::provider::unlock_provider::{
    SubmitUnlockParJs, SubmitUnlockResJs, UnlockParJs, UnlockProvider, UnlockResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

use super::mock_to_sign;

pub struct UnlockProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UnlockProvider for UnlockProviderMock {
    async fn txs(&self, pars: UnlockParJs) -> Result<UnlockResJs, FrError> {
        let algod = algod();

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(UnlockResJs {
            to_sign: mock_to_sign(&algod, &investor_address).await?,
        })
    }

    async fn submit(&self, _: SubmitUnlockParJs) -> Result<SubmitUnlockResJs, FrError> {
        req_delay().await;

        Ok(SubmitUnlockResJs {})
    }
}
