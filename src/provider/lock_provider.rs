use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::FrError,
    js::{common::SignedTxFromJs, to_sign_js::ToSignJs},
};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait LockProvider {
    async fn txs(&self, pars: LockParJs) -> Result<LockResJs, FrError>;
    async fn submit(&self, pars: SubmitLockParJs) -> Result<SubmitLockResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct LockParJs {
    pub dao_id: String,
    pub investor_address: String,
    pub share_count: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LockResJs {
    pub to_sign: ToSignJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitLockParJs {
    // Set if user isn't opted in yet (follows bridge_opt_in_to_apps_if_needed)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitLockResJs {}
