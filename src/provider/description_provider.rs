use anyhow::Result;
use async_trait::async_trait;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DescriptionProvider {
    async fn get(&self, id: String) -> Result<String, FrError>;
}

#[wasm_bindgen]
pub async fn description(pars: String) -> Result<String, FrError> {
    log_wrap_new("description", pars, async move |pars| {
        providers()?.description.get(pars).await
    })
    .await
}
