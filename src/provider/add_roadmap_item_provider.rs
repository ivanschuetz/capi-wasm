use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::js::common::SignedTxFromJs;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AddRoadmapItemProvider {
    async fn txs(&self, pars: AddRoadmapItemParJs) -> Result<AddRoadmapItemResJs>;
    async fn submit(&self, pars: SubmitAddRoadmapItemParJs) -> Result<SubmitAddRoadmapItemResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddRoadmapItemParJs {
    pub creator_address: String,
    pub dao_id: String,
    pub title: String,
    pub date: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddRoadmapItemResJs {
    pub to_sign: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitAddRoadmapItemParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitAddRoadmapItemResJs {
    pub tx_id: String,
}
