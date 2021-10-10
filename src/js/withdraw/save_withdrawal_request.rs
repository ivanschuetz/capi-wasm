use crate::{
    dependencies::{api, environment},
    js::{
        common::{parse_bridge_pars, to_bridge_res},
        withdraw::load_requests::withdrawal_req_to_view_data,
    },
    service::str_to_algos::algos_str_to_microalgos,
};
use anyhow::Result;
use make::api::model::WithdrawalRequestInputs;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::load_requests::WithdrawalRequestViewData;

// TODO rename in bridge_save_withdrawal_request

#[wasm_bindgen]
pub async fn bridge_send_withdrawal_request(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_send_withdrawal_request, pars: {:?}", pars);
    to_bridge_res(_bridge_send_withdrawal_request(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_send_withdrawal_request(pars: SaveWithdrawParJs) -> Result<SaveWithdrawResJs> {
    log::debug!("_bridge_send_withdrawal_request, pars: {:?}", pars);

    let api = api(&environment());

    let saved_request = api
        .submit_withdrawal_request(&WithdrawalRequestInputs {
            project_id: pars.project_id,
            amount: algos_str_to_microalgos(&pars.withdrawal_amount)?,
            description: pars.withdrawal_descr,
        })
        .await?;

    log::debug!("Backend saved the request: {:?}", saved_request);

    // TODO return withdrawal list to refresh UI (in the future only most recent x entries)
    Ok(SaveWithdrawResJs {
        saved_request: withdrawal_req_to_view_data(
            saved_request,
            // request was just created so no one has voted yet. TODO refactor formatting
            "0 %".to_owned(),
            false,
        )?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaveWithdrawParJs {
    pub project_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub withdrawal_descr: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SaveWithdrawResJs {
    pub saved_request: WithdrawalRequestViewData,
}
