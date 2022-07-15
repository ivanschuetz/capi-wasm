use super::{mock_dao_for_users_view_data, mock_to_sign, req_delay};
use crate::{
    error::FrError,
    provider::update_data_provider::{
        SubmitUpdateDataParJs, UpdatableDataParJs, UpdatableDataResJs, UpdateDataParJs,
        UpdateDataPassthroughJs, UpdateDataProvider, UpdateDataResJs,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::{api::fetcher::Fetcher, dependencies::fetcher};
use data_encoding::BASE64;
use mbase::dependencies::algod;

pub struct UpdateDataProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderMock {
    async fn get(&self, _: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        let fetcher = fetcher();

        req_delay().await;

        // just a convienient source for our data
        let mock_dao = mock_dao_for_users_view_data()?;

        let image_bytes = fetcher
            .get("https://ipfs.io/ipfs/bafybeidqjugltb4jayljxd3kigzrypqppr7lwwd67zgac2bqndxejo3irm/img")
            .await?;
        let image_bytes_base64 = BASE64.encode(&image_bytes);

        Ok(UpdatableDataResJs {
            project_name: mock_dao.name,
            project_desc: Some("My project description".to_owned()),
            share_price: "1_000".to_owned(),
            image_base64: Some(image_bytes_base64),
            social_media_url: "https://twitter.com/foobardoesntexist".to_owned(),
        })
    }

    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs, FrError> {
        let algod = algod();
        let owner = pars.owner.parse().map_err(Error::msg)?;

        let mock_dao = mock_dao_for_users_view_data()?;

        req_delay().await;

        Ok(UpdateDataResJs {
            to_sign: mock_to_sign(&algod, &owner).await?,
            pt: UpdateDataPassthroughJs {
                dao_id: mock_dao.app_id,
            },
        })
    }

    async fn submit(&self, _: SubmitUpdateDataParJs) -> Result<()> {
        req_delay().await;

        Ok(())
    }
}
