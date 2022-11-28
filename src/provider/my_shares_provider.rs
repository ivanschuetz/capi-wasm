use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait MySharesProvider {
    async fn get(&self, pars: MySharesParJs) -> Result<MySharesResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct MySharesParJs {
    pub dao_id: String,
    pub my_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct MySharesResJs {
    pub total: String,
    pub free: String,
    pub locked: String,
}

#[wasm_bindgen(js_name=myShares)]
pub async fn my_shares(pars: MySharesParJs) -> Result<MySharesResJs, FrError> {
    log_wrap_new("my_shares", pars, async move |pars| {
        providers()?.my_shares.get(pars).await
    })
    .await
}
