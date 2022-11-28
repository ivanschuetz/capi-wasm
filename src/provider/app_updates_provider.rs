use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AppUpdatesProvider {
    async fn get(&self, pars: CheckForUpdatesParJs) -> Result<CheckForUpdatesResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CheckForUpdatesParJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CheckForUpdatesResJs {
    pub current_approval_version: String,
    pub current_clear_version: String,

    pub update_data: Option<UpdateDataJs>, // set if there's an update
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct UpdateDataJs {
    pub new_approval_version: String,
    pub new_clear_version: String,
}

#[wasm_bindgen(js_name=checkForUpdates)]
pub async fn check_for_updates(
    pars: CheckForUpdatesParJs,
) -> Result<CheckForUpdatesResJs, FrError> {
    log_wrap_new("check_for_updates", pars, async move |pars| {
        providers()?.app_updates.get(pars).await
    })
    .await
}
