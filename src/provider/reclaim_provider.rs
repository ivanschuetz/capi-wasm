use crate::{js::{common::SignedTxFromJs, to_sign_js::ToSignJs}, error::FrError};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ReclaimProvider {
    async fn txs(&self, pars: ReclaimParJs) -> Result<ReclaimResJs, FrError>;
    async fn submit(&self, pars: SubmitReclaimParJs) -> Result<SubmitReclaimResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReclaimParJs {
    pub dao_id: String,
    pub investor_address: String,
    pub share_amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReclaimResJs {
    pub to_sign: ToSignJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitReclaimParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitReclaimResJs {}
