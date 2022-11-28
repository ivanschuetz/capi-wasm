use anyhow::Result;
use async_trait::async_trait;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new, model::dao_js::DaoJs};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DaoProvider {
    async fn get(&self, dao_id_str: String) -> Result<DaoJs, FrError>;
}

#[wasm_bindgen(js_name=loadDao)]
pub async fn load_dao(pars: String) -> Result<DaoJs, FrError> {
    log_wrap_new("load_dao", pars, async move |pars| {
        providers()?.dao.get(pars).await
    })
    .await
}
