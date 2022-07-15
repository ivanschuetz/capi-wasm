use crate::{
    dependencies::funds_asset_specs,
    model::dao_js::{DaoJs, ToDaoJs},
    provider::dao_provider::DaoProvider,
};
use anyhow::Result;
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::load_dao;
use mbase::dependencies::algod;

pub struct DaoUserViewProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DaoProvider for DaoUserViewProviderDef {
    async fn get(&self, dao_id_str: String) -> Result<DaoJs> {
        let algod = algod();

        let dao_id = dao_id_str.parse()?;

        let dao = load_dao(&algod, dao_id).await?;

        Ok(dao.to_js(&funds_asset_specs()?)?)
    }
}
