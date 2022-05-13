use crate::provider::mock::{mock_js_tx, req_delay};
use crate::provider::update_app_provider::{
    SubmitUpdateAppParJs, SubmitUpdateAppResJs, UpdateAppProvider, UpdateDaoAppParJs,
    UpdateDaoAppResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct UpdateAppProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateAppProvider for UpdateAppProviderMock {
    async fn txs(&self, pars: UpdateDaoAppParJs) -> Result<UpdateDaoAppResJs> {
        let algod = algod();

        let owner = pars.owner.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(UpdateDaoAppResJs {
            to_sign: mock_js_tx(&algod, &owner).await?,
        })
    }

    async fn submit(&self, _: SubmitUpdateAppParJs) -> Result<SubmitUpdateAppResJs> {
        req_delay().await;

        Ok(SubmitUpdateAppResJs {})
    }
}
