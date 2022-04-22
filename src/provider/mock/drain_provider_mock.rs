use super::{mock_js_txs, mock_msgpack_tx, req_delay};
use crate::provider::drain_provider::{
    DrainParJs, DrainProvider, DrainResJs, SubmitDrainParJs, SubmitDrainPassthroughParJs,
    SubmitDrainResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::algod;

pub struct DrainProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DrainProvider for DrainProviderMock {
    async fn txs(&self, pars: DrainParJs) -> Result<DrainResJs> {
        let algod = algod();
        let drainer_address = pars.drainer_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(DrainResJs {
            to_sign: mock_js_txs(&algod, &drainer_address).await?,
            pt: SubmitDrainPassthroughParJs {
                drain_tx_msg_pack: mock_msgpack_tx(&algod, &drainer_address).await?,
                capi_share_tx_msg_pack: mock_msgpack_tx(&algod, &drainer_address).await?,
                dao_id: "12312132".to_owned(),
            },
        })
    }

    async fn submit(&self, _: SubmitDrainParJs) -> Result<SubmitDrainResJs> {
        req_delay().await;

        Ok(SubmitDrainResJs {
            new_customer_escrow_balance: "12312".to_owned(),
            new_app_balance: "11111".to_owned(),
        })
    }
}
