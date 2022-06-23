use super::{mock_to_sign, req_delay};
use crate::provider::claim_provider::{
    ClaimParJs, ClaimProvider, ClaimResJs, SubmitClaimParJs, SubmitClaimResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct ClaimProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ClaimProvider for ClaimProviderMock {
    async fn txs(&self, pars: ClaimParJs) -> Result<ClaimResJs> {
        let algod = algod();

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(ClaimResJs {
            to_sign: mock_to_sign(&algod, investor_address).await?,
        })
    }

    async fn submit(&self, _: SubmitClaimParJs) -> Result<SubmitClaimResJs> {
        req_delay().await;

        Ok(SubmitClaimResJs {})
    }
}
