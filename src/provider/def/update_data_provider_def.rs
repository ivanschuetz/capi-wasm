use crate::dependencies::{api, capi_deps};
use crate::js::common::{signed_js_tx_to_signed_tx1, to_my_algo_tx1};
use crate::provider::def::create_dao_provider_def::maybe_upload_image;
use crate::provider::update_data_provider::{
    SubmitUpdateDataParJs, SubmitUpdateDataResJs, UpdatableDataParJs, UpdatableDataResJs,
    UpdateDataParJs, UpdateDataPassthroughJs, UpdateDataProvider, UpdateDataResJs,
};
use algonaut::core::Address;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::api::image_api::ImageApi;
use base::dependencies::image_api;
use base::flows::create_dao::setup_dao_specs::{CompressedImage, HashableString};
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::update_data::update_data::{
    submit_update_data, update_data, UpdatableDaoData, UpdateDaoDataSigned,
};
use data_encoding::BASE64;
use mbase::api::version::{Version, VersionedAddress};
use mbase::dependencies::algod;
use mbase::models::dao_id::DaoId;
use mbase::models::hash::GlobalStateHash;
use mbase::state::dao_app_state::dao_global_state;

pub struct UpdateDataProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderDef {
    async fn get(&self, pars: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        let algod = algod();
        let api = api();
        let image_api = image_api();
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

        let app_state = dao_global_state(&algod, dao_id.0).await?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        let image_bytes = match dao.specs.image_hash {
            Some(hash) => {
                let bytes = image_api.get_image(&hash.as_api_id()).await?;
                let base64 = BASE64.encode(&bytes);
                Some(base64)
            }
            None => None,
        };

        Ok(UpdatableDataResJs {
            owner: app_state.owner.to_string(),

            customer_escrow: app_state.customer_escrow.address.to_string(),
            customer_escrow_version: app_state.customer_escrow.version.0.to_string(),

            project_name: app_state.project_name,
            project_desc: app_state.project_desc.map(|h| h.as_str()),
            share_price: app_state.share_price.to_string(),

            image_bytes,
            social_media_url: app_state.social_media_url,
        })
    }

    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs> {
        let algod = algod();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;
        let owner = pars.owner.parse().map_err(Error::msg)?;

        // TODO escrow versioning
        // we're currently saving only the addresses, so don't have the programs to lsig
        // so we've to store the version too (although it could be skipped by just trying all available versions against the address, which seems very inefficient)
        // and use this version to retrieve the program
        // the teal has to be updated to store the version, either in the same field as the address or a separate field with all the escrow's versions

        let image = pars.image.map(CompressedImage::new);
        let image_hash = image.as_ref().map(|i| i.hash());

        let to_sign = update_data(
            &algod,
            &owner,
            dao_id.0,
            &UpdatableDaoData {
                customer_escrow: VersionedAddress::new(
                    parse_addr(pars.customer_escrow)?,
                    parse_int(pars.customer_escrow_version)?,
                ),
                project_name: pars.project_name,
                project_desc: pars.project_desc.map(|d| d.hash()),
                image_hash: image_hash.clone(),
                social_media_url: pars.social_media_url,
                owner,
            },
        )
        .await?;

        Ok(UpdateDataResJs {
            to_sign: to_my_algo_tx1(&to_sign.update).map_err(Error::msg)?,
            pt: UpdateDataPassthroughJs {
                dao_id: dao_id.to_string(),
                image: image.map(|i| i.bytes()),
                image_hash: image_hash.map(|h| h.bytes()),
            },
        })
    }

    async fn submit(&self, pars: SubmitUpdateDataParJs) -> Result<SubmitUpdateDataResJs> {
        let algod = algod();
        let image_api = image_api();

        let dao_id = pars.pt.dao_id.parse::<DaoId>().map_err(Error::msg)?;
        let image = pars.pt.image.map(CompressedImage::new);
        let image_hash = match pars.pt.image_hash {
            Some(bytes) => Some(GlobalStateHash::from_bytes(bytes)?),
            None => None,
        };

        let tx_id = submit_update_data(
            &algod,
            UpdateDaoDataSigned {
                update: signed_js_tx_to_signed_tx1(&pars.tx)?,
            },
        )
        .await?;

        // Note that it's required to upload the image after the DAO update, because the image api checks the hash in the app's global state.
        let (maybe_image_url, maybe_image_upload_error) =
            maybe_upload_image(&algod, &image_api, tx_id, dao_id.0, image, image_hash).await?;

        Ok(SubmitUpdateDataResJs {
            image_url: maybe_image_url,
            image_upload_error: maybe_image_upload_error,
        })
    }
}

fn parse_int(str: String) -> Result<Version> {
    Ok(Version(str.parse().map_err(Error::msg)?))
}

fn parse_addr(s: String) -> Result<Address> {
    s.parse().map_err(Error::msg)
}
