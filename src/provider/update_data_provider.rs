use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::js::common::SignedTxFromJs;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateDataProvider {
    async fn get(&self, pars: UpdatableDataParJs) -> Result<UpdatableDataResJs>;
    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs>;
    async fn submit(&self, pars: SubmitUpdateDataParJs) -> Result<SubmitUpdateDataResJs>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatableDataParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdatableDataResJs {
    pub owner: String,

    pub customer_escrow: String,

    pub customer_escrow_version: String,

    pub project_name: String,
    pub project_desc: Option<String>,
    pub share_price: String,

    pub image_bytes: Option<String>, // base64 - js image cropper library expects it
    pub social_media_url: String,
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataParJs {
    pub dao_id: String,
    pub owner: String,

    pub customer_escrow: String,
    pub customer_escrow_version: String,

    pub project_name: String,
    pub project_desc: Option<String>,
    pub share_price: String,

    pub image: Option<Vec<u8>>,
    pub social_media_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDataResJs {
    pub to_sign: Vec<Value>,
    pub pt: UpdateDataPassthroughJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUpdateDataParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: UpdateDataPassthroughJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataPassthroughJs {
    pub dao_id: String,
    pub image: Option<Vec<u8>>,
    pub image_hash: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUpdateDataResJs {
    pub image_url: Option<String>,
    pub image_upload_error: Option<String>,
}
