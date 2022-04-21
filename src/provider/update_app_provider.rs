use crate::js::common::SignedTxFromJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateAppProvider {
    async fn update(&self, pars: UpdateDaoAppParJs) -> Result<UpdateDaoAppResJs>;
    async fn submit(&self, pars: SubmitUpdateAppParJs) -> Result<SubmitUpdateAppResJs>;
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDaoAppParJs {
    pub dao_id: String,
    pub owner: String,
    pub approval_version: String,
    pub clear_version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDaoAppResJs {
    pub to_sign: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUpdateAppParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUpdateAppResJs {}
