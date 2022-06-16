use crate::js::common::SignedTxFromJs;
use crate::js::to_sign_js::ToSignJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RekeyProvider {
    async fn txs(&self, pars: RekeyParJs) -> Result<RekeyResJs>;
    async fn submit(&self, pars: SubmitRekeyParJs) -> Result<SubmitRekeyResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct RekeyParJs {
    pub dao_id: String,
    pub auth_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RekeyResJs {
    pub to_sign: ToSignJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRekeyParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitRekeyResJs {}
