use super::providers;
use crate::{error::FrError, js::bridge::log_wrap_new};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BalanceProvider {
    async fn get(&self, pars: BalanceParJs) -> Result<BalanceResJs, FrError>;
    async fn get_balance_change(
        &self,
        pars: BalanceChangeParJs,
    ) -> Result<BalanceChangeResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct BalanceParJs {
    pub address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct BalanceResJs {
    pub balance_algos: String,
    pub balance_funds_asset: String,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct BalanceChangeParJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct BalanceChangeResJs {
    pub change: String,
}

#[wasm_bindgen]
pub async fn balance(pars: BalanceParJs) -> Result<BalanceResJs, FrError> {
    log_wrap_new("balance", pars, async move |pars| {
        providers()?.balance.get(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=getBalanceChange)]
pub async fn get_balance_change(pars: BalanceChangeParJs) -> Result<BalanceChangeResJs, FrError> {
    log_wrap_new("get_balance_change", pars, async move |pars| {
        providers()?.balance.get_balance_change(pars).await
    })
    .await
}
    