use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WyreProvider {
    async fn reserve(&self, pars: WyreReserveParsJs) -> Result<WyreReserveResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct WyreReserveParsJs {
    pub address: String,
    pub dst_currency: String,
    pub dst_amount: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct WyreReserveResJs {
    pub url: String,
    pub reservation: String,
}

#[wasm_bindgen(js_name=reserveWyre)]
pub async fn reserve_wyre(pars: WyreReserveParsJs) -> Result<WyreReserveResJs, FrError> {
    log_wrap_new("reserve_wyre", pars, async move |pars| {
        providers()?.wyre.reserve(pars).await
    })
    .await
}
