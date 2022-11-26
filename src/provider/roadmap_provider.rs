use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RoadmapProvider {
    async fn get(&self, pars: GetRoadmapParJs) -> Result<GetRoadmapResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRoadmapParJs {
    pub creator_address: String,
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
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
