use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, to_sign_js::ToSignJs},
};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait OptinToAppProvider {
    async fn txs(&self, pars: OptInToAppParJs) -> Result<OptInToAppResJs, FrError>;
}

// TODO rename structs in BuyShares*
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct OptInToAppParJs {
    pub app_id: String,
    pub investor_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct OptInToAppResJs {
    pub to_sign: Option<ToSignJs>,
}

#[wasm_bindgen(js_name=optInToAppsIfNeeded)]
pub async fn opt_in_to_apps_if_needed(pars: OptInToAppParJs) -> Result<OptInToAppResJs, FrError> {
    log_wrap_new("opt_in_to_apps_if_needed", pars, async move |pars| {
        providers()?.app_optin.txs(pars).await
    })
    .await
}
