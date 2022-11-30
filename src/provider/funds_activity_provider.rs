use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait FundsActivityProvider {
    async fn get(&self, pars: LoadFundsActivityParJs) -> Result<LoadFundsActivityResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct LoadFundsActivityParJs {
    pub dao_id: String,
    pub max_results: Option<String>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct LoadFundsActivityResJs {
    pub entries: Vec<FundsActivityViewData>,
}

unsafe impl Send for LoadFundsActivityResJs {}

#[derive(Tsify, Debug, Clone, Serialize, PartialEq, Eq)]
#[tsify(into_wasm_abi)]
pub struct FundsActivityViewData {
    pub amount: String,
    pub short_amount: String,
    pub fee: String,
    pub amount_without_fee: String,
    pub short_amount_without_fee: String,
    pub is_income: String, // false: spending
    // not used currently TODO: later, when we've more identifiable types (e.g. investment, payment), change this in type_id (localization in react)
    pub type_label: String,
    pub description: String,
    pub date: String,
    pub tx_id: String,
    pub tx_link: String,
    pub address: String,
}

#[wasm_bindgen(js_name=loadFundsActivity)]
pub async fn load_funds_activity(
    pars: LoadFundsActivityParJs,
) -> Result<LoadFundsActivityResJs, FrError> {
    log_wrap_new("load_funds_activity", pars, async move |pars| {
        providers()?.funds_activity.get(pars).await
    })
    .await
}
