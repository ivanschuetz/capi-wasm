use super::{mock_to_sign, req_delay};
use crate::provider::optin_to_app_provider::{
    OptInToAppParJs, OptInToAppResJs, OptinToAppProvider,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct OptinToAppProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OptinToAppProvider for OptinToAppProviderMock {
    async fn txs(&self, pars: OptInToAppParJs) -> Result<OptInToAppResJs> {
        let algod = algod();

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(OptInToAppResJs {
            to_sign: Some(mock_to_sign(&algod, &investor_address).await?),
        })
    }
}
