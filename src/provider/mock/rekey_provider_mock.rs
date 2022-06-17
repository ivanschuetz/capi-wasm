use super::{mock_to_sign, req_delay};
use crate::{
    error::FrError,
    provider::rekey_provider::{
        RekeyParJs, RekeyProvider, RekeyResJs, SubmitRekeyParJs, SubmitRekeyResJs,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct RekeyProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RekeyProvider for RekeyProviderMock {
    async fn txs(&self, pars: RekeyParJs) -> Result<RekeyResJs, FrError> {
        let algod = algod();

        let auth = pars.auth_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(RekeyResJs {
            to_sign: mock_to_sign(&algod, &auth).await?,
        })
    }

    async fn submit(&self, _pars: SubmitRekeyParJs) -> Result<SubmitRekeyResJs> {
        req_delay().await;

        Ok(SubmitRekeyResJs {})
    }
}
