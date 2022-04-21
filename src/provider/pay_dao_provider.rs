use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::js::common::SignedTxFromJs;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PayDaoProvider {
    async fn txs(&self, pars: PayDaoParJs) -> Result<PayDaoResJs>;
    async fn submit(&self, pars: SubmitPayDaoParJs) -> Result<SubmitPayDaoResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct PayDaoParJs {
    pub customer_address: String,
    pub customer_escrow_address: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PayDaoResJs {
    pub to_sign: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitPayDaoParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitPayDaoResJs {}
