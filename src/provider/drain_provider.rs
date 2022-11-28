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
pub trait DrainProvider {
    async fn txs(&self, pars: DrainParJs) -> Result<DrainResJs, FrError>;
    async fn submit(&self, pars: SubmitDrainParJs) -> Result<SubmitDrainResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
// TODO this can be optimized passing the already loaded dao from JS
// to not load the dao again
// (we'd have to use the complete dao instance - drain needs lsig)
pub struct DrainParJs {
    pub dao_id: String,
    pub drainer_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct DrainResJs {
    pub to_sign: ToSignJs,
    pub pt: SubmitDrainPassthroughParJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
/// The assets creation signed transactions and the specs to create the dao
pub struct SubmitDrainParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitDrainPassthroughParJs,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct SubmitDrainPassthroughParJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitDrainResJs {
    pub new_app_balance: String,
}

#[wasm_bindgen]
pub async fn drain(pars: DrainParJs) -> Result<DrainResJs, FrError> {
    log_wrap_new("drain", pars, async move |pars| {
        providers()?.drain.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitDrain)]
pub async fn submit_drain(pars: SubmitDrainParJs) -> Result<SubmitDrainResJs, FrError> {
    log_wrap_new("submit_drain", pars, async move |pars| {
        providers()?.drain.submit(pars).await
    })
    .await
}
