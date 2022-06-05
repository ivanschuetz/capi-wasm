use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::js::{common::SignedTxFromJs, to_sign_js::ToSignJs};

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
    pub to_sign: ToSignJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitPayDaoParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitPayDaoResJs {}
