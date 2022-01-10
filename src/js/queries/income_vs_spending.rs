use crate::{
    js::common::{parse_bridge_pars, to_bridge_res},
    service::str_to_algos::microalgos_to_algos_str,
    teal::programs,
};
use anyhow::Result;
use core::{
    dependencies::{algod, indexer},
    flows::create_project::storage::load_project::load_project,
    queries::{received_payments::received_payments, withdrawals::withdrawals},
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

    let project = load_project(
        &algod,
        &indexer,
        &pars.project_id.parse()?,
        &programs().escrows,
    )
    .await?;

    let mut income = received_payments(&indexer, project.customer_escrow.address()).await?;
    log::debug!("Income: {:?}", income);
    income.sort_by(|p1, p2| p1.date.cmp(&p2.date));

    let mut spending = withdrawals(&indexer, &project.creator, &pars.project_uuid.parse()?).await?;
    log::debug!("Spending: {:?}", income);
    spending.sort_by(|p1, p2| p1.date.cmp(&p2.date));

    let income_data_points = income.into_iter().map(|payment| ChartDataPoint {
        date: payment.date.timestamp().to_string(),
        value: microalgos_to_algos_str(payment.amount),
    });

    let spending_data_points = spending.into_iter().map(|withdrawal| ChartDataPoint {
        date: withdrawal.date.timestamp().to_string(),
        value: microalgos_to_algos_str(withdrawal.amount),
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
    // TODO remove: use only project_id
    pub project_uuid: String,
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
