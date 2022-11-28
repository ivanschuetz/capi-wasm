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
pub trait ReclaimProvider {
    async fn txs(&self, pars: ReclaimParJs) -> Result<ReclaimResJs, FrError>;
    async fn submit(&self, pars: SubmitReclaimParJs) -> Result<SubmitReclaimResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct ReclaimParJs {
    pub dao_id: String,
    pub investor_address: String,
    pub share_amount: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ReclaimResJs {
    pub to_sign: ToSignJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitReclaimParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitReclaimResJs {}

#[wasm_bindgen]
pub async fn reclaim(pars: ReclaimParJs) -> Result<ReclaimResJs, FrError> {
    log_wrap_new("reclaim", pars, async move |pars| {
        providers()?.reclaim.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitReclaim)]
pub async fn submit_reclaim(pars: SubmitReclaimParJs) -> Result<SubmitReclaimResJs, FrError> {
    log_wrap_new("submit_reclaim", pars, async move |pars| {
        providers()?.reclaim.submit(pars).await
    })
    .await
}
