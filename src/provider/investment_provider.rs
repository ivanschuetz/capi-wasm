use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait InvestmentProvider {
    async fn available_shares(
        &self,
        pars: AvailableSharesParJs,
    ) -> Result<AvailableSharesResJs, FrError>;
    async fn get_investor_data(
        &self,
        pars: LoadInvestorParJs,
    ) -> Result<LoadInvestorResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct LoadInvestorParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct LoadInvestorResJs {
    pub investor_shares_count: String,
    pub investor_share: String, // relative to the dao's income
    pub investor_already_retrieved_amount: String,
    pub investor_claimable_dividend: String,
    pub investor_claimable_dividend_microalgos: String, // passthrough
    pub investor_locked_shares: String,
    pub investor_unlocked_shares: String,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct AvailableSharesParJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct AvailableSharesResJs {
    pub available_shares_number: String,
    pub available_shares: String,
}

#[wasm_bindgen(js_name=loadAvailableShares)]
pub async fn load_available_shares(
    pars: AvailableSharesParJs,
) -> Result<AvailableSharesResJs, FrError> {
    log_wrap_new("load_available_shares", pars, async move |pars| {
        providers()?.investment.available_shares(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=loadInvestment)]
pub async fn load_investment(pars: LoadInvestorParJs) -> Result<LoadInvestorResJs, FrError> {
    log_wrap_new("load_investment", pars, async move |pars| {
        providers()?.investment.get_investor_data(pars).await
    })
    .await
}
