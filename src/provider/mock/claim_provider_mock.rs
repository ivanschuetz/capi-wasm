use super::{mock_js_txs, req_delay};
use crate::provider::claim_provider::{
    ClaimParJs, ClaimProvider, ClaimResJs, SubmitClaimParJs, SubmitClaimPassthroughParJs,
    SubmitClaimResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::algod;

pub struct ClaimProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ClaimProvider for ClaimProviderMock {
    async fn txs(&self, pars: ClaimParJs) -> Result<ClaimResJs> {
        let algod = algod();

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(ClaimResJs {
            to_sign: mock_js_txs(&algod, investor_address).await?,
            pt: SubmitClaimPassthroughParJs {
                maybe_drain_tx_msg_pack: None,
                maybe_capi_share_tx_msg_pack: None,
            },
        })
    }

    async fn submit(&self, _: SubmitClaimParJs) -> Result<SubmitClaimResJs> {
        req_delay().await;

        Ok(SubmitClaimResJs {})
    }
}
