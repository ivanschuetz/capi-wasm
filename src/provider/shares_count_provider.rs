use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait SharesCountProvider {
    async fn get(&self, pars: GetUserSharesCountParJs) -> Result<String, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct GetUserSharesCountParJs {
    pub address: String,
    pub shares_asset_id: String,
}

#[wasm_bindgen(js_name=getUserSharesCount)]
pub async fn get_user_shares_count(pars: GetUserSharesCountParJs) -> Result<String, FrError> {
    log_wrap_new("get_user_shares_count", pars, async move |pars| {
        providers()?.shares_count.get(pars).await
    })
    .await
}
