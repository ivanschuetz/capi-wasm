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
pub trait UpdateAppProvider {
    async fn txs(&self, pars: UpdateDaoAppParJs) -> Result<UpdateDaoAppResJs, FrError>;
    async fn submit(&self, pars: SubmitUpdateAppParJs) -> Result<SubmitUpdateAppResJs, FrError>;
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct UpdateDaoAppParJs {
    pub dao_id: String,
    pub owner: String,
    pub approval_version: String,
    pub clear_version: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct UpdateDaoAppResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitUpdateAppParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitUpdateAppResJs {}

#[wasm_bindgen(js_name=updateAppTxs)]
pub async fn update_app_txs(pars: UpdateDaoAppParJs) -> Result<UpdateDaoAppResJs, FrError> {
    log_wrap_new("update_app_txs", pars, async move |pars| {
        providers()?.update_app.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitUpdateApp)]
pub async fn submit_update_app(
    pars: SubmitUpdateAppParJs,
) -> Result<SubmitUpdateAppResJs, FrError> {
    log_wrap_new("submit_update_app", pars, async move |pars| {
        providers()?.update_app.submit(pars).await
    })
    .await
}
