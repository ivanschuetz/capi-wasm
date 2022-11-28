use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, common::SignedTxFromJs, to_sign_js::ToSignJs},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UnlockProvider {
    async fn txs(&self, pars: UnlockParJs) -> Result<UnlockResJs, FrError>;
    async fn submit(&self, pars: SubmitUnlockParJs) -> Result<SubmitUnlockResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct UnlockParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct UnlockResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
/// The assets creation signed transactions and the specs to create the dao
pub struct SubmitUnlockParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitUnlockResJs {}

#[wasm_bindgen]
pub async fn unlock(pars: UnlockParJs) -> Result<UnlockResJs, FrError> {
    log_wrap_new("unlock", pars, async move |pars| {
        providers()?.unlock.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitUnlock)]
pub async fn submit_unlock(pars: SubmitUnlockParJs) -> Result<SubmitUnlockResJs, FrError> {
    log_wrap_new("submit_unlock", pars, async move |pars| {
        providers()?.unlock.submit(pars).await
    })
    .await
}
