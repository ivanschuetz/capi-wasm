use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, common::SignedTxFromJs, to_sign_js::ToSignJs},
};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait LockProvider {
    async fn txs(&self, pars: LockParJs) -> Result<LockResJs, FrError>;
    async fn submit(&self, pars: SubmitLockParJs) -> Result<SubmitLockResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct LockParJs {
    pub dao_id: String,
    pub investor_address: String,
    pub share_count: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct LockResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitLockParJs {
    // Set if user isn't opted in yet (follows bridge_opt_in_to_apps_if_needed)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitLockResJs {}

#[wasm_bindgen]
pub async fn lock(pars: LockParJs) -> Result<LockResJs, FrError> {
    log_wrap_new("lock", pars, async move |pars| {
        providers()?.lock.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitLock)]
pub async fn submit_lock(pars: SubmitLockParJs) -> Result<SubmitLockResJs, FrError> {
    log_wrap_new("submit_lock", pars, async move |pars| {
        providers()?.lock.submit(pars).await
    })
    .await
}
