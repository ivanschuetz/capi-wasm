use super::{mock_to_sign, req_delay};
use crate::{
    error::FrError,
    provider::drain_provider::{
        DrainParJs, DrainProvider, DrainResJs, SubmitDrainParJs, SubmitDrainPassthroughParJs,
        SubmitDrainResJs,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct DrainProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DrainProvider for DrainProviderMock {
    async fn txs(&self, pars: DrainParJs) -> Result<DrainResJs, FrError> {
        let algod = algod();
        let drainer_address = pars.drainer_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(DrainResJs {
            to_sign: mock_to_sign(&algod, &drainer_address).await?,
            pt: SubmitDrainPassthroughParJs {
                dao_id: "12312132".to_owned(),
            },
        })
    }

    async fn submit(&self, _: SubmitDrainParJs) -> Result<SubmitDrainResJs, FrError> {
        req_delay().await;

        Ok(SubmitDrainResJs {
            new_app_balance: "11111".to_owned(),
        })
    }
}
