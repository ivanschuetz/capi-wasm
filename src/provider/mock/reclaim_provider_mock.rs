use crate::provider::mock::req_delay;
use crate::provider::reclaim_provider::{
    ReclaimParJs, ReclaimProvider, ReclaimResJs, SubmitReclaimParJs, SubmitReclaimResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

use super::mock_to_sign;

pub struct ReclaimProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ReclaimProvider for ReclaimProviderMock {
    async fn txs(&self, pars: ReclaimParJs) -> Result<ReclaimResJs> {
        let algod = algod();

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(ReclaimResJs {
            to_sign: mock_to_sign(&algod, &investor_address).await?,
        })
    }

    async fn submit(&self, _: SubmitReclaimParJs) -> Result<SubmitReclaimResJs> {
        req_delay().await;

        Ok(SubmitReclaimResJs {})
    }
}
