use crate::{
    dependencies::api,
    provider::app_updates_provider::{
        AppUpdatesProvider, CheckForUpdatesParJs, CheckForUpdatesResJs, UpdateDataJs,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::{
    api::teal_api::TealApi, api::version::Version, dependencies::algod, models::dao_id::DaoId,
    state::dao_app_state::dao_global_state,
};

pub struct AppUpdatesProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AppUpdatesProvider for AppUpdatesProviderDef {
    async fn get(&self, pars: CheckForUpdatesParJs) -> Result<CheckForUpdatesResJs> {
        let algod = algod();
        let api = api();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

        let state = dao_global_state(&algod, dao_id.0).await?;
        let last_versions = api.last_versions();

        let update_data = if last_versions.app_approval.0 > state.app_approval_version.0
            || last_versions.app_clear.0 > state.app_clear_version.0
        {
            Some(UpdateData {
                new_approval_version: last_versions.app_approval,
                new_clear_version: last_versions.app_clear,
            })
        } else {
            None
        };

        if let Some(update_data) = &update_data {
            log::debug!("There's a new version: {:?}", update_data);
        }

        Ok(CheckForUpdatesResJs {
            current_approval_version: state.app_approval_version.0.to_string(),
            current_clear_version: state.app_clear_version.0.to_string(),
            update_data: update_data.map(update_data_to_js),
        })
    }
}

#[derive(Debug)]
struct UpdateData {
    new_approval_version: Version,
    new_clear_version: Version,
}

fn update_data_to_js(data: UpdateData) -> UpdateDataJs {
    UpdateDataJs {
        new_approval_version: data.new_approval_version.0.to_string(),
        new_clear_version: data.new_clear_version.0.to_string(),
    }
}
