use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait IncomeVsSpendingProvider {
    async fn get(&self, pars: IncomeVsSpendingParJs) -> Result<IncomeVsSpendingResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncomeVsSpendingParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncomeVsSpendingResJs {
    pub points: Vec<ChartDataPointJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartDataPointJs {
    pub date: String,
    pub income: String,
    pub spending: String,
}
