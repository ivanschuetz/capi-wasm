use crate::{
    dependencies::funds_asset_specs,
    js::common::{parse_bridge_pars, to_bridge_res},
    service::str_to_algos::base_units_to_display_units_str,
    teal::programs,
};
use anyhow::Result;
use core::{
    dependencies::{algod, indexer},
    flows::{
        create_project::storage::load_project::load_project, withdraw::withdrawals::withdrawals,
    },
    queries::received_payments::received_payments,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_income_vs_spending(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_income_vs_spending, pars: {:?}", pars);
    to_bridge_res(_bridge_income_vs_spending(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_income_vs_spending(
    pars: IncomeVsSpendingParJs,
) -> Result<IncomeVsSpendingResJs> {
    let algod = algod();
    let indexer = indexer();
    let funds_asset_specs = funds_asset_specs();

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows)
        .await?
        .project;

    let mut income = received_payments(&indexer, project.customer_escrow.address()).await?;
    log::debug!("Income: {:?}", income);
    income.sort_by(|p1, p2| p1.date.cmp(&p2.date));

    let mut spending = withdrawals(
        &algod,
        &indexer,
        &project.creator,
        &project_id,
        &programs().escrows,
    )
    .await?;
    log::debug!("Spending: {:?}", income);
    spending.sort_by(|p1, p2| p1.date.cmp(&p2.date));

    let income_data_points = income.into_iter().map(|payment| ChartDataPoint {
        date: payment.date.timestamp().to_string(),
        value: base_units_to_display_units_str(payment.amount, &funds_asset_specs),
    });

    let spending_data_points = spending.into_iter().map(|withdrawal| ChartDataPoint {
        date: withdrawal.date.timestamp().to_string(),
        value: base_units_to_display_units_str(withdrawal.amount, &funds_asset_specs),
    });

    let flattened = income_data_points
        .clone()
        .chain(spending_data_points.clone());

    Ok(IncomeVsSpendingResJs {
        chart_lines: ChartLines {
            spending: spending_data_points.collect(),
            income: income_data_points.collect(),
        },
        flat_data_points: flattened.collect(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncomeVsSpendingParJs {
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncomeVsSpendingResJs {
    // the data points used to draw the chart lines
    pub chart_lines: ChartLines,
    // the same data as in chart_lines, but flattened: the chart needs this to render the axes
    pub flat_data_points: Vec<ChartDataPoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartLines {
    pub spending: Vec<ChartDataPoint>,
    pub income: Vec<ChartDataPoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartDataPoint {
    date: String,
    value: String,
}
