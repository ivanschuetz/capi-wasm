use crate::provider::mock::{mock_js_txs, req_delay};
use crate::provider::unlock_provider::{
    SubmitUnlockParJs, SubmitUnlockResJs, UnlockParJs, UnlockProvider, UnlockResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::algod;

pub struct UnlockProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UnlockProvider for UnlockProviderMock {
    async fn txs(&self, pars: UnlockParJs) -> Result<UnlockResJs> {
        let algod = algod();

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(UnlockResJs {
            to_sign: mock_js_txs(&algod, &investor_address).await?,
        })
    }

    async fn submit(&self, _: SubmitUnlockParJs) -> Result<SubmitUnlockResJs> {
        req_delay().await;

        Ok(SubmitUnlockResJs {})
    }
}
