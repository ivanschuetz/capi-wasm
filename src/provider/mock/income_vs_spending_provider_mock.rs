use crate::dependencies::funds_asset_specs;
use crate::provider::def::income_vs_spending_provider_def::{
    to_income_vs_spending_res_static_bounds, ChartDataPoint,
};
use crate::provider::income_vs_spending_provider::{
    to_interval_data, IncomeVsSpendingParJs, IncomeVsSpendingProvider, IncomeVsSpendingResJs,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};

use super::req_delay;

pub struct IncomeVsSpendingProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IncomeVsSpendingProvider for IncomeVsSpendingProviderMock {
    async fn get(&self, pars: IncomeVsSpendingParJs) -> Result<IncomeVsSpendingResJs> {
        let funds_asset_specs = funds_asset_specs()?;

        let now = Utc::now();

        let income_data_points = test_income_points(now);
        let spending_data_points = test_spending_points(now);

        let interval_data = to_interval_data(&pars.interval)?;

        req_delay().await;

        to_income_vs_spending_res_static_bounds(
            income_data_points,
            spending_data_points,
            &funds_asset_specs,
            interval_data,
        )
    }
}

#[allow(dead_code)]
fn test_income_points(now: DateTime<Utc>) -> Vec<ChartDataPoint> {
    vec![
        ChartDataPoint {
            date: now - Duration::days(0),
            value: 1_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: now - Duration::days(1),
            value: 5_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: now - Duration::days(3),
            value: 3_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: now - Duration::days(4),
            value: 4_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: now - Duration::days(7),
            value: 5_000_000,
            is_income: true,
        },
    ]
}

#[allow(dead_code)]
fn test_spending_points(now: DateTime<Utc>) -> Vec<ChartDataPoint> {
    vec![
        ChartDataPoint {
            date: now - Duration::days(0),
            value: 1_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: now - Duration::days(2),
            value: 5_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: now - Duration::days(3),
            value: 3_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: now - Duration::days(5),
            value: 4_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: now - Duration::days(6),
            value: 5_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: now - Duration::days(7),
            value: 5_000_000,
            is_income: false,
        },
    ]
}
