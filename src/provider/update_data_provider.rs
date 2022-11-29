use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, common::SignedTxFromJs, to_sign_js::ToSignJs},
    model::ProspectusJs,
};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateDataProvider {
    async fn get(&self, pars: UpdatableDataParJs) -> Result<UpdatableDataResJs, FrError>;
    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs, FrError>;
    async fn submit(&self, pars: SubmitUpdateDataParJs) -> Result<(), FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct UpdatableDataParJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
/// Data to prefill update form
pub struct UpdatableDataResJs {
    pub project_name: String,
    pub project_desc: Option<String>,
    pub share_price: String,

    pub image_base64: Option<String>, // js image cropper library expects base64
    pub social_media_url: String,

    pub prospectus: Option<ProspectusJs>,
    pub min_invest_amount: String,
    pub max_invest_amount: String,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
pub struct UpdateDataParJs {
    pub dao_id: String,
    pub owner: String,

    pub project_name: String,
    pub project_desc_url: Option<String>,
    // TODO remove? not updatable currently
    pub share_price: String,

    pub image: Option<Vec<u8>>,
    pub image_url: Option<String>,
    pub social_media_url: String,

    pub prospectus_url: Option<String>,
    // bytes and hash are OR:
    // if uploading a new prospectus, we have bytes (to generate the hash in rust)
    // if not updating it, we have the hash of the old prospectus
    pub prospectus_bytes: Option<Vec<u8>>,
    pub prospectus_hash: Option<String>,

    pub min_invest_amount: String,
    pub max_invest_amount: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct UpdateDataResJs {
    pub to_sign: ToSignJs,
    pub pt: UpdateDataPassthroughJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitUpdateDataParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: UpdateDataPassthroughJs,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct UpdateDataPassthroughJs {
    pub dao_id: String,
}

/// To pre fill the form to update data
#[wasm_bindgen(js_name=updatableData)]
pub async fn updatable_data(pars: UpdatableDataParJs) -> Result<UpdatableDataResJs, FrError> {
    log_wrap_new("updatable_data", pars, async move |pars| {
        providers()?.update_data.get(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=updateData)]
pub async fn update_data(pars: UpdateDataParJs) -> Result<UpdateDataResJs, FrError> {
    log_wrap_new("update_data", pars, async move |pars| {
        providers()?.update_data.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitUpdateDaoData)]
pub async fn submit_update_dao_data(pars: SubmitUpdateDataParJs) -> Result<(), FrError> {
    log_wrap_new("submit_update_dao_data", pars, async move |pars| {
        providers()?.update_data.submit(pars).await
    })
    .await
}
