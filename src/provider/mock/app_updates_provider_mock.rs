use crate::{
    error::FrError,
    provider::app_updates_provider::{
        AppUpdatesProvider, CheckForUpdatesParJs, CheckForUpdatesResJs, UpdateDataJs,
    },
};
use anyhow::Result;
use async_trait::async_trait;

use super::req_delay;

pub struct AppUpdatesProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AppUpdatesProvider for AppUpdatesProviderMock {
    async fn get(&self, _: CheckForUpdatesParJs) -> Result<CheckForUpdatesResJs, FrError> {
        req_delay().await;

        Ok(CheckForUpdatesResJs {
            current_approval_version: "1".to_owned(),
            current_clear_version: "1".to_owned(),
            update_data: Some(UpdateDataJs {
                new_approval_version: "2".to_owned(),
                new_clear_version: "2".to_owned(),
            }),
        })
    }
}
