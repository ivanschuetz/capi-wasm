use crate::js::{common::SignedTxFromJs, to_sign_js::ToSignJs};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UnlockProvider {
    async fn txs(&self, pars: UnlockParJs) -> Result<UnlockResJs>;
    async fn submit(&self, pars: SubmitUnlockParJs) -> Result<SubmitUnlockResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnlockParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnlockResJs {
    pub to_sign: ToSignJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUnlockParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUnlockResJs {}
