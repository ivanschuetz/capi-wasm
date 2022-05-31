use anyhow::Result;
use async_trait::async_trait;

use crate::model::dao_js::DaoJs;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DaoProvider {
    async fn get(&self, dao_id_str: String) -> Result<DaoJs>;
}
