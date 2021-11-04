use core::state::withdrawal_app_state::withdrawal_slot_voter_state;

use crate::{
    dependencies::{algod, api, environment},
    js::{
        common::{parse_bridge_pars, to_bridge_res},
        withdraw::load_requests::{
            withdrawal_req_to_view_data_fetch_votes, WithdrawalRequestViewData,
        },
    },
};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_withdrawal_requests_for_voters(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_withdrawal_requests, pars: {:?}", pars);
    to_bridge_res(_bridge_load_withdrawal_requests_for_voters(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_withdrawal_requests_for_voters(
    pars: LoadWithdrawalRequestsForVotersParJs,
) -> Result<LoadWithdrawalRequestsForVotersResJs> {
    let env = &environment();
    let api = api(env);
    let algod = algod(env);
    // let requests = load_withdrawal_requests(&algod, &api, &pars.project_id).await?;

    let investor_address = pars.user_address.parse().map_err(Error::msg)?;

    let project = api.load_project(&pars.project_id).await?;
    let requests = api.load_withdrawal_requests(&pars.project_id).await?;

    let mut reqs_view_data = vec![];
    for req in requests {
        let slot_id = req.slot_id.parse()?; // hmm why parse here?
        let investor_state =
            withdrawal_slot_voter_state(&algod, &investor_address, slot_id).await?;

        reqs_view_data.push(WithdrawalRequestForVotersViewData {
            req: withdrawal_req_to_view_data_fetch_votes(&algod, &req, &project).await?,
            user_voted: investor_state.did_vote_in_current_round(),
        });
    }
    Ok(LoadWithdrawalRequestsForVotersResJs {
        requests: reqs_view_data,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadWithdrawalRequestsForVotersParJs {
    pub project_id: String,
    pub user_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadWithdrawalRequestsForVotersResJs {
    pub requests: Vec<WithdrawalRequestForVotersViewData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WithdrawalRequestForVotersViewData {
    pub req: WithdrawalRequestViewData,
    pub user_voted: bool,
}
