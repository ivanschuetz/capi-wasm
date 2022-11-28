use crate::{error::FrError, js::bridge::log_wrap_new};
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait FundsRaisingProvider {
    async fn data(&self, pars: FundsRaisingParsJs) -> Result<FundsRaisingResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct FundsRaisingParsJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct FundsRaisingResJs {
    pub raised_number: String,
    pub raised: String,
    pub state: FundsRaisingStateJs,
    pub goal_exceeded_percentage: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum FundsRaisingStateJs {
    Raising,
    GoalReached,
    GoalNotReached,
    GoalExceeded,
}

#[derive(Debug, Clone)]
pub enum FundsRaisingState {
    Raising,
    GoalReached,
    GoalNotReached,
    GoalExceeded(Decimal), // decimal: percentage by which the goal was exceeded
}

#[wasm_bindgen(js_name=raisedFunds)]
pub async fn raised_funds(pars: FundsRaisingParsJs) -> Result<FundsRaisingResJs, FrError> {
    log_wrap_new("raised_funds", pars, async move |pars| {
        providers()?.raised.data(pars).await
    })
    .await
}
