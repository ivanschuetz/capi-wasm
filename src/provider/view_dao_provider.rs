use crate::model::dao_js::DaoJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ViewDaoProvider {
    async fn get(&self, pars: ViewDaoParJs) -> Result<ViewDaoResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct ViewDaoParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ViewDaoResJs {
    pub dao: DaoJs,
    pub shares_available: String,
    pub investors_share: String,
    pub available_funds: String,
    pub customer_payment_deeplink: String,
}
