use crate::dependencies::{api, capi_deps, funds_asset_specs, FundsAssetSpecs};
use crate::provider::income_vs_spending_provider::{
    ChartDataPointJs, ChartLines, IncomeVsSpendingParJs, IncomeVsSpendingProvider,
    IncomeVsSpendingResJs,
};
use crate::service::str_to_algos::base_units_to_display_units;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base::dependencies::indexer;
use base::funds::FundsAmount;
use base::{
    dependencies::algod,
    flows::{create_dao::storage::load_dao::load_dao, withdraw::withdrawals::withdrawals},
    queries::received_payments::all_received_payments,
};
use chrono::{DateTime, Duration, Timelike, Utc};
use std::convert::TryInto;
use std::ops::Div;

pub struct IncomeVsSpendingProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IncomeVsSpendingProvider for IncomeVsSpendingProviderDef {
    async fn get(&self, pars: IncomeVsSpendingParJs) -> Result<IncomeVsSpendingResJs> {
        let algod = algod();
        let api = api();
        let indexer = indexer();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        let mut income = all_received_payments(
            &indexer,
            &dao.app_address(),
            dao.customer_escrow.address(),
            dao.funds_asset_id,
        )
        .await?;
        income.sort_by(|p1, p2| p1.date.cmp(&p2.date));

        let mut spending =
            withdrawals(&algod, &indexer, &dao.owner, dao_id, &api, &capi_deps).await?;
        spending.sort_by(|p1, p2| p1.date.cmp(&p2.date));

        let income_data_points: Vec<ChartDataPoint> = income
            .into_iter()
            .map(|payment| ChartDataPoint {
                date: payment.date,
                value: payment.amount.val(),
            })
            .collect();

        let spending_data_points: Vec<ChartDataPoint> = spending
            .into_iter()
            .map(|withdrawal| ChartDataPoint {
                date: withdrawal.date,
                value: withdrawal.amount.val(),
            })
            .collect();

        to_income_vs_spending_res(
            &income_data_points,
            &spending_data_points,
            &funds_asset_specs,
        )
    }
}

// pub to be shared with the mock provider
pub fn to_income_vs_spending_res(
    income: &[ChartDataPoint],
    spending: &[ChartDataPoint],
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<IncomeVsSpendingResJs> {
    let income_bounds = determine_min_max_local_bounds(&income);
    let spending_bounds = determine_min_max_local_bounds(&spending);

    let bounds_without_none: Vec<DateBounds> = vec![income_bounds, spending_bounds]
        .into_iter()
        .filter_map(|opt| opt)
        .collect();

    // TODO determine dynamically based on min max bounds (e.g. if total time is ~1 month, 1 week interval)
    // let grouping_interval = Duration::minutes(15);
    let grouping_interval = Duration::days(1);

    match determine_min_max_bounds(&bounds_without_none) {
        Some(bounds) => {
            let bounds = DateBounds {
                // days (for axis) start at 00:00 if using days interval, hours at 00 (if using hours/minutes..)
                // TODO calculate dynamically
                min: start_of_day(bounds.min)?,
                // min: start_of_hour(bounds.min)?,
                max: bounds.max,
            };

            let income_data_points_js = aggregate_and_format_data_points(
                &income,
                bounds.min,
                bounds.max,
                grouping_interval,
                &funds_asset_specs,
            )?;

            let spending_data_points_js = aggregate_and_format_data_points(
                &spending,
                bounds.min,
                bounds.max,
                grouping_interval,
                &funds_asset_specs,
            )?;

            let mut flattened_js = income_data_points_js.clone();
            flattened_js.extend(spending_data_points_js.clone());

            Ok(IncomeVsSpendingResJs {
                chart_lines: ChartLines {
                    spending: spending_data_points_js,
                    income: income_data_points_js,
                },
                flat_data_points: flattened_js,
            })
        }
        // No min max dates -> nothing to display on the chart
        None => Ok(IncomeVsSpendingResJs {
            chart_lines: ChartLines {
                spending: vec![],
                income: vec![],
            },
            flat_data_points: vec![],
        }),
    }
}

/// Returns min max dates of data_points
/// Assumes that data_points are sorted ascendingly by date
/// If there's only one point, it returns min and max with the same value
/// If the first and last data point have the same date, it also returns min and max with this same value
fn determine_min_max_local_bounds(data_points: &[ChartDataPoint]) -> Option<DateBounds> {
    match (data_points.first(), data_points.last()) {
        (Some(first), Some(last)) => Some(DateBounds {
            min: first.date,
            max: last.date,
        }),
        _ => None, // empty -> no bounds
    }
}

fn determine_min_max_bounds(local_bounds: &[DateBounds]) -> Option<DateBounds> {
    if let Some(first) = local_bounds.first() {
        let mut min = first.min;
        let mut max = first.max;

        for b in local_bounds {
            min = b.min.min(min);
            max = b.max.max(max);
        }

        Some(DateBounds { min, max })
    } else {
        None // empty -> no bounds
    }
}

#[allow(dead_code)]
fn start_of_day(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(date
        .with_hour(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set day 0 on date"))?
        .with_minute(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set min 0 on date"))?
        .with_second(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set second 0 on date"))?
        .with_nanosecond(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set nanosecond 0 on date"))?)
}

#[allow(dead_code)]
fn start_of_hour(date: DateTime<Utc>) -> Result<DateTime<Utc>> {
    Ok(date
        .with_minute(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set min 0 on date"))?
        .with_second(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set second 0 on date"))?
        .with_nanosecond(0)
        .ok_or_else(|| anyhow!("Unexpected: couldn't set nanosecond 0 on date"))?)
}

#[derive(Debug, Clone)]
struct DateBounds {
    min: DateTime<Utc>,
    max: DateTime<Utc>,
}

pub fn aggregate_and_format_data_points(
    points: &[ChartDataPoint],
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    interval_length: Duration,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<Vec<ChartDataPointJs>> {
    let length = interval_count(end_time, start_time, interval_length)?;
    let mut values: Vec<u64> = vec![0; length + 1]; // +1 -> inclusive range

    for point in points {
        let interval_index = interval_count(point.date, start_time, interval_length)?;
        values[interval_index] = values[interval_index]
            .checked_add(point.value)
            .ok_or(anyhow!(
                "Overflow adding value and point.value: {:?}",
                point.value
            ))?;
    }

    let mut data_points = vec![];

    for (index, value) in values.into_iter().enumerate() {
        let data_point_js = create_data_point_js(
            start_time,
            index.try_into()?,
            interval_length,
            value,
            funds_asset_specs,
        )?;
        log::debug!("mapped index: {index}, value: {value:?} to js point: {data_point_js:?}");
        data_points.push(data_point_js);
    }

    Ok(data_points)
}

fn interval_count(
    date: DateTime<Utc>,
    start: DateTime<Utc>,
    interval_length: Duration,
) -> Result<usize> {
    log::debug!(
        "calc inverval count: date: {date:?}, start: {start:?}, length: {interval_length:?}"
    );
    Ok((date - start)
        .num_seconds()
        .div(interval_length.num_seconds())
        .try_into()?)
}

fn create_data_point_js(
    start_time: DateTime<Utc>,
    interval_index: i64,
    interval: Duration,
    value: u64,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<ChartDataPointJs> {
    let date = start_time
        .checked_add_signed(Duration::seconds(
            interval
                .num_seconds()
                .checked_mul(interval_index)
                .ok_or(anyhow!(
                    "Error multiplying interval: {interval} with index: {interval_index}"
                ))?,
        ))
        .ok_or(anyhow!("Error adding duration to start time: {start_time}"))?;
    let value = base_units_to_display_units(FundsAmount::new(value), funds_asset_specs);

    Ok(ChartDataPointJs {
        date: date.timestamp().to_string(),
        value: value.to_string(),
    })
}

#[derive(Debug, Clone)]
pub struct ChartDataPoint {
    pub date: DateTime<Utc>,
    pub value: u64,
}
