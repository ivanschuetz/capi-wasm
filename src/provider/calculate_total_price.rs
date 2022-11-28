use crate::{error::FrError, js::bridge::log_wrap_new};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CalculateTotalPriceProvider {
    async fn get(
        &self,
        pars: CalculateTotalPriceParJs,
    ) -> Result<CalculateTotalPriceResJs, FrError>;

    async fn max_funds(
        &self,
        pars: CalculateMaxFundsParJs,
    ) -> Result<CalculateMaxFundsResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CalculateTotalPriceParJs {
    pub shares_amount: String,
    pub available_shares: String,
    pub share_supply: String,
    pub locked_shares: Option<String>,
    pub investors_share: String,
    pub share_price: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CalculateTotalPriceResJs {
    pub total_price: String,
    pub total_price_number: String,
    pub profit_percentage: String,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CalculateMaxFundsParJs {
    pub shares_amount: String,
    pub share_price: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CalculateMaxFundsResJs {
    pub total_price: String,
    pub total_price_number: String,
}

#[wasm_bindgen(js_name=calculateSharesPrice)]
pub async fn calculate_shares_price(
    pars: CalculateTotalPriceParJs,
) -> Result<CalculateTotalPriceResJs, FrError> {
    log_wrap_new("calculate_shares_price", pars, async move |pars| {
        providers()?.calculate_total_price.get(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=calculateMaxFunds)]
pub async fn calculate_max_funds(
    pars: CalculateMaxFundsParJs,
) -> Result<CalculateMaxFundsResJs, FrError> {
    log_wrap_new("calculate_max_funds", pars, async move |pars| {
        providers()?.calculate_total_price.max_funds(pars).await
    })
    .await
}
