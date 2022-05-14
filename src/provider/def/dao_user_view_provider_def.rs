use crate::{
    dependencies::{api, capi_deps, funds_asset_specs},
    model::{
        dao_for_users::dao_to_dao_for_users,
        dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData},
    },
    provider::dao_user_view_provider::DaoUserViewProvider,
    ImageHashExt,
};
use anyhow::Result;
use async_trait::async_trait;
use base::{dependencies::image_api, flows::create_dao::storage::load_dao::load_dao};
use mbase::dependencies::algod;

pub struct DaoUserViewProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DaoUserViewProvider for DaoUserViewProviderDef {
    async fn get(&self, dao_id_str: String) -> Result<DaoForUsersViewData> {
        let algod = algod();
        let image_api = image_api();
        let api = api();
        let capi_deps = capi_deps()?;

        let dao_id = dao_id_str.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        Ok(dao_for_users_to_view_data(
            dao_to_dao_for_users(
                &dao,
                &dao_id,
                dao.specs
                    .image_hash
                    .clone()
                    .map(|h| h.as_api_url(&image_api)),
            ),
            &funds_asset_specs()?,
        ))
    }
}
