use crate::js::{common::SignedTxFromJs, to_sign_js::ToSignJs};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ClaimProvider {
    async fn txs(&self, pars: ClaimParJs) -> Result<ClaimResJs>;
    async fn submit(&self, pars: SubmitClaimParJs) -> Result<SubmitClaimResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaimParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClaimResJs {
    pub to_sign: ToSignJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitClaimParJs {
    pub investor_address_for_diagnostics: String,
    pub dao_id_for_diagnostics: String,

    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitClaimResJs {}
