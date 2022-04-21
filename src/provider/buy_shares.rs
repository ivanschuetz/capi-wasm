use crate::js::common::SignedTxFromJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BuySharesProvider {
    async fn txs(&self, pars: InvestParJs) -> Result<InvestResJs>;
    async fn submit(&self, pars: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs>;
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct InvestParJs {
    pub dao_id: String,
    pub share_count: String,
    pub investor_address: String,
    // not set if the user was already opted in (checked in previous step)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitBuySharesPassthroughParJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitBuySharesParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitBuySharesPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBuySharesPassthroughParJs {
    pub dao_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitBuySharesResJs {
    pub message: String,
}
