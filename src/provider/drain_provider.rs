use crate::js::{common::SignedTxFromJs, to_sign_js::ToSignJs};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DrainProvider {
    async fn txs(&self, pars: DrainParJs) -> Result<DrainResJs>;
    async fn submit(&self, pars: SubmitDrainParJs) -> Result<SubmitDrainResJs>;
}

// TODO this can be optimized passing the already loaded dao from JS
// to not load the dao again
// (we'd have to use the complete dao instance - drain needs lsig)
#[derive(Debug, Clone, Deserialize)]
pub struct DrainParJs {
    pub dao_id: String,
    pub drainer_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DrainResJs {
    pub to_sign: ToSignJs,
    pub pt: SubmitDrainPassthroughParJs,
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitDrainParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitDrainPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitDrainPassthroughParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitDrainResJs {
    pub new_app_balance: String,
}
