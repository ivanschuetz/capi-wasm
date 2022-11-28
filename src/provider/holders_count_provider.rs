use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HoldersCountProvider {
    async fn get(&self, pars: HoldersCountParJs) -> Result<HoldersCountResJs, FrError>;

    /// returns "up"/"down"/"eq" if holders went up/down/staid the same, and "unknown" if nothing to compare with
    /// note that here we use local storage (this is currently inconsistent with other places with "change"),
    /// thus "unknown" if there's no older entry to compare with.
    async fn change(&self, pars: HoldersChangeParJs) -> Result<HoldersChangeResJs, FrError>;
}

// TODO use dao_id (convention?), in HoldersChangeParJs too

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct HoldersCountParJs {
    pub asset_id: String,
    pub app_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct HoldersCountResJs {
    pub count: String,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct HoldersChangeParJs {
    pub asset_id: String,
    pub app_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct HoldersChangeResJs {
    pub change: String,
}

#[wasm_bindgen(js_name=holdersCount)]
pub async fn holders_count(pars: HoldersCountParJs) -> Result<HoldersCountResJs, FrError> {
    log_wrap_new("holders_count", pars, async move |pars| {
        providers()?.holders_count.get(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=holdersChange)]
pub async fn holders_change(pars: HoldersChangeParJs) -> Result<HoldersChangeResJs, FrError> {
    log_wrap_new("holders_change", pars, async move |pars| {
        providers()?.holders_count.change(pars).await
    })
    .await
}
