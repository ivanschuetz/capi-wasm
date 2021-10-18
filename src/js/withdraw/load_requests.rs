use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
    server::api::Api,
    service::str_to_algos::microalgos_to_algos,
};
use algonaut::algod::v2::Algod;
use anyhow::Result;
use make::{
    api::model::SavedWithdrawalRequest, flows::create_project::model::Project,
    withdrawal_app_state::votes_global_state,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_withdrawal_requests(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_withdrawal_requests, pars: {:?}", pars);
    to_bridge_res(_bridge_load_withdrawal_requests(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_withdrawal_requests(
    pars: LoadWithdrawalRequestsParJs,
) -> Result<LoadWithdrawalRequestsResJs> {
    let env = &environment();
    let api = api(env);
    let algod = algod(env);
    let requests = load_withdrawal_requests(&algod, &api, &pars.project_id).await?;

    Ok(LoadWithdrawalRequestsResJs { requests })
}

pub async fn load_withdrawal_requests(
    algod: &Algod,
    api: &Api,
    project_id: &String,
) -> Result<Vec<WithdrawalRequestViewData>> {
    let project = api.load_project(&project_id).await?;

    let requests = api.load_withdrawal_requests(&project_id).await?;

    let mut reqs_view_data = vec![];
    for req in requests {
        let votes = get_votes(algod, req.slot_id.parse()?).await?;
        let votes_str = format_votes(&project, votes);
        reqs_view_data.push(withdrawal_req_to_view_data(
            req,
            votes_str,
            votes >= project.specs.vote_threshold,
        )?);
    }
    Ok(reqs_view_data)
}

async fn get_votes(algod: &Algod, slot_id: u64) -> Result<u64> {
    let slot_app = algod.application_information(slot_id).await?;
    Ok(votes_global_state(&slot_app).unwrap_or_else(|| 0))
}

pub async fn get_votes_percentage(
    algod: &Algod,
    project: &Project,
    slot_id: u64,
) -> Result<String> {
    // TODO Decimal
    let percentage = get_votes(algod, slot_id).await? as f64 / project.specs.shares.count as f64;
    Ok(format!("{} %", percentage * 100 as f64))
}

pub fn format_votes(project: &Project, count: u64) -> String {
    // TODO Decimal
    let percentage = count as f64 / project.specs.shares.count as f64;
    format!("{} %", percentage * 100 as f64)
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadWithdrawalRequestsParJs {
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadWithdrawalRequestsResJs {
    pub requests: Vec<WithdrawalRequestViewData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WithdrawalRequestViewData {
    pub amount: String,
    pub description: String,
    pub date: String,
    pub votes: String,
    pub can_withdraw: String,
    pub complete: String,

    // passthrough model data
    pub request_id: String,
    pub slot_id: String,
    pub amount_not_formatted: String,
}

pub fn withdrawal_req_to_view_data(
    req: SavedWithdrawalRequest,
    votes: String,
    can_withdraw: bool,
) -> Result<WithdrawalRequestViewData> {
    Ok(WithdrawalRequestViewData {
        amount: format!("{} Algo", microalgos_to_algos(req.amount).to_string()),
        description: req.description,
        date: req.date.to_rfc2822(),
        votes,
        can_withdraw: can_withdraw.to_string(),
        request_id: req.id,
        slot_id: req.slot_id,
        amount_not_formatted: req.amount.to_string(), // microalgos
        complete: req.complete.to_string(),
    })
}
