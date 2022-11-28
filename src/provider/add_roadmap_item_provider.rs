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
pub trait AddRoadmapItemProvider {
    async fn txs(&self, pars: AddRoadmapItemParJs) -> Result<AddRoadmapItemResJs, FrError>;
    async fn submit(
        &self,
        pars: SubmitAddRoadmapItemParJs,
    ) -> Result<SubmitAddRoadmapItemResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct AddRoadmapItemParJs {
    pub creator_address: String,
    pub dao_id: String,
    pub title: String,
    pub date: String,
    pub parent: Option<String>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct AddRoadmapItemResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitAddRoadmapItemParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitAddRoadmapItemResJs {
    pub tx_id: String,
}

#[wasm_bindgen(js_name=addRoadmapItem)]
pub async fn add_roadmap_item(pars: AddRoadmapItemParJs) -> Result<AddRoadmapItemResJs, FrError> {
    log_wrap_new("add_roadmap_item", pars, async move |pars| {
        providers()?.add_roadmap_item.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitAddRoadmapItem)]
pub async fn submit_add_roadmap_item(
    pars: SubmitAddRoadmapItemParJs,
) -> Result<SubmitAddRoadmapItemResJs, FrError> {
    log_wrap_new("submit_add_roadmap_item", pars, async move |pars| {
        providers()?.add_roadmap_item.submit(pars).await
    })
    .await
}
