use super::{mock_address, mock_dao_for_users_view_data, mock_js_tx, req_delay};
use crate::provider::update_data_provider::{
    SubmitUpdateDataParJs, SubmitUpdateDataResJs, UpdatableDataParJs, UpdatableDataResJs,
    UpdateDataParJs, UpdateDataProvider, UpdateDataResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::algod;

pub struct UpdateDataProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderMock {
    async fn get(&self, _: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        req_delay().await;

        // just a convienient source for our data
        let mock_dao = mock_dao_for_users_view_data()?;

        Ok(UpdatableDataResJs {
            owner: mock_address()?.to_string(),
            customer_escrow: mock_address()?.to_string(),
            customer_escrow_version: "1".to_owned(),
            project_name: mock_dao.name,
            project_desc: "My project description".to_owned(),
            share_price: "1_000".to_owned(),
            logo_url: "https://foo.bar.com/mylogo_url200x300".to_owned(),
            social_media_url: "https://twitter.com/foobardoesntexist".to_owned(),
        })
    }

    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs> {
        let algod = algod();
        let owner = pars.owner.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(UpdateDataResJs {
            to_sign: mock_js_tx(&algod, &owner).await?,
        })
    }

    async fn submit(&self, _: SubmitUpdateDataParJs) -> Result<SubmitUpdateDataResJs> {
        req_delay().await;

        Ok(SubmitUpdateDataResJs {})
    }
}
