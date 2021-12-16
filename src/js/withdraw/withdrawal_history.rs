use crate::{
    dependencies::{api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
    server::api::Api,
    service::str_to_algos::microalgos_to_algos,
};
use anyhow::Result;
use core::api::model::SavedWithdrawal;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_withdrawals(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_withdrawals, pars: {:?}", pars);
    to_bridge_res(_bridge_load_withdrawals(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_withdrawals(pars: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs> {
    let env = &environment();
    let api = api(env);
    let entries = load_withdrawals(&api, &pars.project_id).await?;

    Ok(LoadWithdrawalResJs { entries })
}

pub async fn load_withdrawals(api: &Api, project_id: &str) -> Result<Vec<WithdrawalViewData>> {
    let entries = api.load_withdrawal_requests(project_id).await?;
    let mut reqs_view_data = vec![];
    for req in entries {
        reqs_view_data.push(withdrawal_to_view_data(&req)?);
    }
    Ok(reqs_view_data)
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadWithdrawalParJs {
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadWithdrawalResJs {
    pub entries: Vec<WithdrawalViewData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WithdrawalViewData {
    pub amount: String,
    pub description: String,
    pub date: String,

    // passthrough model data
    pub request_id: String,
    pub amount_not_formatted: String,
}

pub fn withdrawal_to_view_data(req: &SavedWithdrawal) -> Result<WithdrawalViewData> {
    Ok(WithdrawalViewData {
        amount: format!("{} Algo", microalgos_to_algos(req.amount).to_string()),
        description: req.description.clone(),
        date: req.date.to_rfc2822(),
        request_id: req.id.clone(),
        amount_not_formatted: req.amount.to_string(), // microalgos
    })
}
