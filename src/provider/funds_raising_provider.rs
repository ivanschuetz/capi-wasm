use crate::error::FrError;
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait FundsRaisingProvider {
    async fn data(&self, pars: FundsRaisingParsJs) -> Result<FundsRaisingResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct FundsRaisingParsJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
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
