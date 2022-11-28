use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use mbase::date_util::DateTimeExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait IncomeVsSpendingProvider {
    async fn get(&self, pars: IncomeVsSpendingParJs) -> Result<IncomeVsSpendingResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct IncomeVsSpendingParJs {
    pub dao_id: String,
    pub interval: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct IncomeVsSpendingResJs {
    pub points: Vec<ChartDataPointJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartDataPointJs {
    pub date: String,
    pub income: String,
    pub spending: String,
}

#[derive(Debug, Clone)]
pub struct IntervalData {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub interval: Duration,
}

pub fn to_interval_data(interval_str: &str) -> Result<IntervalData> {
    let end = Utc::now().zero_time()?;
    match interval_str {
        "days7" => Ok(IntervalData {
            start: end - Duration::days(6),
            end,
            interval: Duration::days(1),
        }),
        "months3" => Ok(IntervalData {
            start: end - Duration::weeks(11),
            end,
            interval: Duration::weeks(1),
        }),
        "year" => Ok(IntervalData {
            start: end - Duration::weeks(47),
            end,
            interval: Duration::weeks(4),
        }),
        _ => Err(anyhow!("Not supported interval str: {:?}", interval_str)),
    }
}

#[wasm_bindgen(js_name=incomeVsSpending)]
pub async fn income_vs_spending(
    pars: IncomeVsSpendingParJs,
) -> Result<IncomeVsSpendingResJs, FrError> {
    log_wrap_new("income_vs_spending", pars, async move |pars| {
        providers()?.income_vs_spending.get(pars).await
    })
    .await
}
