use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WithdrawalHistoryProvider {
    async fn get(&self, pars: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct LoadWithdrawalParJs {
    pub dao_id: String,
    pub creator_address: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct LoadWithdrawalResJs {
    pub entries: Vec<WithdrawalViewData>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct WithdrawalViewData {
    pub amount: String,
    pub description: String,
    pub date: String,

    pub tx_id: String,
    pub tx_link: String,

    /// passthrough model data
    pub amount_not_formatted: String,
}

#[wasm_bindgen(js_name=loadWithdrawals)]
pub async fn load_withdrawals(pars: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs, FrError> {
    log_wrap_new("load_withdrawals", pars, async move |pars| {
        providers()?.withdrawals_history.get(pars).await
    })
    .await
}
