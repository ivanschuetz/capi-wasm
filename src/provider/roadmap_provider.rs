use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RoadmapProvider {
    async fn get(&self, pars: GetRoadmapParJs) -> Result<GetRoadmapResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct GetRoadmapParJs {
    pub creator_address: String,
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct GetRoadmapResJs {
    pub items: Vec<RoadmapItemJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapItemJs {
    pub item_type: String,       // "item" | "header"
    pub tx_id: Option<String>,   // set if type == "item"
    pub tx_link: Option<String>, // set if type == "item"
    pub date: Option<String>,    // set if type == "item"
    pub text: String,
}

#[wasm_bindgen(js_name=loadRoadmap)]
pub async fn load_roadmap(pars: GetRoadmapParJs) -> Result<GetRoadmapResJs, FrError> {
    log_wrap_new("load_roadmap", pars, async move |pars| {
        providers()?.roadmap.get(pars).await
    })
    .await
}
