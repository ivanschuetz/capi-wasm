use crate::js::common::SignedTxFromJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub to_sign: Vec<Value>,
    pub pt: SubmitClaimPassthroughParJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitClaimParJs {
    pub investor_address_for_diagnostics: String,
    pub dao_id_for_diagnostics: String,

    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitClaimPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitClaimPassthroughParJs {
    // set if a drain tx is necessary
    pub maybe_drain_tx_msg_pack: Option<Vec<u8>>,
    pub maybe_capi_share_tx_msg_pack: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitClaimResJs {}
