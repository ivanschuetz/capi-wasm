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
pub trait ClaimProvider {
    async fn txs(&self, pars: ClaimParJs) -> Result<ClaimResJs, FrError>;
    async fn submit(&self, pars: SubmitClaimParJs) -> Result<SubmitClaimResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct ClaimParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ClaimResJs {
    pub to_sign: ToSignJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitClaimParJs {
    pub investor_address_for_diagnostics: String,
    pub dao_id_for_diagnostics: String,

    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitClaimResJs {}

#[wasm_bindgen]
pub async fn claim(pars: ClaimParJs) -> Result<ClaimResJs, FrError> {
    log_wrap_new("claim", pars, async move |pars| {
        providers()?.claim.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitClaim)]
pub async fn submit_claim(pars: SubmitClaimParJs) -> Result<SubmitClaimResJs, FrError> {
    log_wrap_new("submit_claim", pars, async move |pars| {
        providers()?.claim.submit(pars).await
    })
    .await
}
