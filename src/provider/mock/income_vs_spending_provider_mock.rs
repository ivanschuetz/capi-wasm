use crate::dependencies::funds_asset_specs;
use crate::provider::def::income_vs_spending_provider_def::{
    to_income_vs_spending_res, ChartDataPoint,
};
use crate::provider::income_vs_spending_provider::{
    to_interval_data, IncomeVsSpendingParJs, IncomeVsSpendingProvider, IncomeVsSpendingResJs,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};

use super::req_delay;

pub struct IncomeVsSpendingProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IncomeVsSpendingProvider for IncomeVsSpendingProviderMock {
    async fn get(&self, pars: IncomeVsSpendingParJs) -> Result<IncomeVsSpendingResJs> {
        let funds_asset_specs = funds_asset_specs()?;

        let income_data_points = test_income_points();
        let spending_data_points = test_spending_points();

        let interval_data = to_interval_data(&pars.interval)?;

        req_delay().await;

        to_income_vs_spending_res(
            income_data_points,
            spending_data_points,
            &funds_asset_specs,
            interval_data.interval,
        )
    }
}

fn create_test_date(year: i32, month: u32, day: u32, hour: u32, min: u32) -> DateTime<Utc> {
    Utc.ymd(year, month, day).and_hms_milli(hour, min, 0, 0)
}

#[allow(dead_code)]
fn test_income_points() -> Vec<ChartDataPoint> {
    vec![
        ChartDataPoint {
            date: create_test_date(2022, 2, 10, 10, 30),
            value: 1_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 12, 12, 0),
            value: 5_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 15, 9, 0),
            value: 3_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 15, 18, 30),
            value: 4_000_000,
            is_income: true,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 16, 20, 15),
            value: 5_000_000,
            is_income: true,
        },
    ]
}

#[allow(dead_code)]
fn test_spending_points() -> Vec<ChartDataPoint> {
    vec![
        ChartDataPoint {
            date: create_test_date(2022, 2, 8, 10, 30),
            value: 1_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 8, 12, 0),
            value: 5_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 14, 9, 0), // appears as 13 10:30 UTC value 3, and then a 14 10:30 UTC wth value 0
            value: 3_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 15, 18, 30), // appears as 15 10:30 value 4
            value: 4_000_000,
            is_income: false,
        },
        ChartDataPoint {
            date: create_test_date(2022, 2, 18, 20, 15),
            value: 5_000_000,
            is_income: false,
        },
    ]
}
