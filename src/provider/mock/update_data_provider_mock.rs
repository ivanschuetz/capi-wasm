use super::{mock_dao_for_users_view_data, mock_to_sign, req_delay};
use crate::{
    error::FrError,
    provider::update_data_provider::{
        SubmitUpdateDataParJs, SubmitUpdateDataResJs, UpdatableDataParJs, UpdatableDataResJs,
        UpdateDataParJs, UpdateDataPassthroughJs, UpdateDataProvider, UpdateDataResJs,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::{api::image_api::ImageApi, dependencies::image_api};
use data_encoding::BASE64;
use mbase::dependencies::algod;

pub struct UpdateDataProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderMock {
    async fn get(&self, _: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        let image_api = image_api();

        req_delay().await;

        // just a convienient source for our data
        let mock_dao = mock_dao_for_users_view_data()?;

        // we've to fetch these from the api because we want to test a real image in the UI and hardcoded bytes here is too much
        // this may break if the api delted the image
        // TODO consider allowing image fetch to fail (everywhere) - so e.g. image api being down doesn't make everything fail
        let image_bytes = image_api
            .get_image("xqXI6IBs1tSlfNAARFXiFeq4376WBrv6Wcexg2C2gG4=")
            .await?;
        let image_bytes_base64 = BASE64.encode(&image_bytes);

        Ok(UpdatableDataResJs {
            project_name: mock_dao.name,
            project_desc: Some("My project description".to_owned()),
            share_price: "1_000".to_owned(),
            image_bytes: Some(image_bytes_base64),
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
                image: None,
                image_hash: None,
            },
        })
    }

    async fn submit(&self, _: SubmitUpdateDataParJs) -> Result<SubmitUpdateDataResJs> {
        req_delay().await;

        Ok(SubmitUpdateDataResJs {
            image_url: None,
            image_upload_error: None,
        })
    }
}
