use crate::provider::holders_count_provider::{
    HoldersCountParJs, HoldersCountProvider, HoldersCountResJs,
};
use algonaut::core::to_app_address;
use anyhow::Result;
use async_trait::async_trait;
use base::queries::shares_distribution::holders_count;
use mbase::dependencies::indexer;

pub struct HoldersCountProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HoldersCountProvider for HoldersCountProviderDef {
    async fn get(&self, pars: HoldersCountParJs) -> Result<HoldersCountResJs> {
        let indexer = indexer();

        let asset_id = pars.asset_id.parse()?;

        let app_id = pars.app_id.parse()?;

        let app_address = to_app_address(app_id);

        Ok(HoldersCountResJs {
            count: holders_count(&indexer, asset_id, &app_address)
                .await?
                .to_string(),
        })
    }
}
