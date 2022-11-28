use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DividendsProvider {
    async fn get(&self, pars: DividendsParJs) -> Result<String, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct DividendsParJs {
    pub investor_address: String,
    pub dao_id: String,
}

#[wasm_bindgen(js_name=myDividend)]
pub async fn my_dividend(pars: DividendsParJs) -> Result<String, FrError> {
    log_wrap_new("my_dividend", pars, async move |pars| {
        providers()?.dividend.get(pars).await
    })
    .await
}
