use crate::error::FrError;
use crate::js::bridge::log_wrap_new;
use crate::js::common::SignedTxFromJs;
use crate::js::to_sign_js::ToSignJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RekeyProvider {
    async fn txs(&self, pars: RekeyParJs) -> Result<RekeyResJs, FrError>;
    async fn submit(&self, pars: SubmitRekeyParJs) -> Result<SubmitRekeyResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct RekeyParJs {
    pub dao_id: String,
    pub auth_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct RekeyResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitRekeyParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitRekeyResJs {}

#[wasm_bindgen(js_name=rekeyOwner)]
pub async fn rekey_owner(pars: RekeyParJs) -> Result<RekeyResJs, FrError> {
    log_wrap_new("rekey_owner", pars, async move |pars| {
        providers()?.rekey.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitRekeyOwner)]
pub async fn submit_rekey_owner(pars: SubmitRekeyParJs) -> Result<SubmitRekeyResJs, FrError> {
    log_wrap_new("submit_rekey_owner", pars, async move |pars| {
        providers()?.rekey.submit(pars).await
    })
    .await
}
