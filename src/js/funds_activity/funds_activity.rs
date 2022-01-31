use crate::{
    js::{common::{parse_bridge_pars, to_bridge_res}, explorer_links::explorer_tx_id_link_env},
    teal::programs, service::str_to_algos::{microalgos_to_algos_str},
};
use anyhow::{Error, Result};
use core::{
    dependencies::{algod, indexer}, flows::create_project::storage::load_project::load_project, 
    queries::funds_activity::{funds_activity, FundsActivityEntryType}
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_funds_activity(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_funds_activity, pars: {:?}", pars);
    to_bridge_res(_bridge_load_funds_activity(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_funds_activity(pars: LoadFundsActivityParJs) -> Result<LoadFundsActivityResJs> {
    let algod = algod();
    let indexer = indexer();

    let creator = pars.creator_address.parse().map_err(Error::msg)?;

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows)
        .await?
        .project;

    let mut activity_entries = funds_activity(&algod, &indexer, &creator, &project_id, &project.customer_escrow.address(), &programs().escrows).await?;
    // sort descendingly by date
    activity_entries.sort_by(|p1, p2| p2.date.cmp(&p1.date));

    let mut view_data_entries = vec![];
    for entry in activity_entries {
        view_data_entries.push(FundsActivityViewData {
            amount: microalgos_to_algos_str(entry.amount),
            is_income: match entry.type_ {
                FundsActivityEntryType::Income => "true",
                FundsActivityEntryType::Spending => "false",
            }.to_owned(),
            description: entry.description,
            date: entry.date.to_rfc2822(),
            tx_id: entry.tx_id.to_string(),
            tx_link: explorer_tx_id_link_env(&entry.tx_id),
        });
    }

    Ok(LoadFundsActivityResJs { entries: view_data_entries })
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadFundsActivityParJs {
    pub project_id: String,
    pub creator_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadFundsActivityResJs {
    pub entries: Vec<FundsActivityViewData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FundsActivityViewData {
    pub amount: String,
    pub is_income: String, // false: spending
    pub description: String,
    pub date: String,
    pub tx_id: String,
    pub tx_link: String,
}
