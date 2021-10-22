use crate::{
    dependencies::{algod, api, environment},
    js::{
        common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs},
        withdraw::load_requests::withdrawal_req_to_view_data,
    },
    service::str_to_algos::algos_str_to_microalgos,
};
use anyhow::{anyhow, Result};
use make::{
    api::model::WithdrawalRequestInputs,
    flows::withdraw::init_withdrawal::{submit_init_withdrawal, InitWithdrawalSigned},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::load_requests::WithdrawalRequestViewData;

#[wasm_bindgen]
pub async fn bridge_submit_init_withdrawal_request(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_init_withdrawal_request, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_init_withdrawal_request(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_init_withdrawal_request(
    pars: SubmitInitWithdrawalRequestParJs,
) -> Result<SubmitInitWithdrawalRequestResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    if pars.txs.len() != 1 {
        return Err(anyhow!(
            "Unexpected signed withdraw txs length: {}",
            pars.txs.len()
        ));
    }
    let init_withdrawal_slot_app_call_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;

    let withdraw_tx_id = submit_init_withdrawal(
        &algod,
        &InitWithdrawalSigned {
            init_withdrawal_slot_app_call_tx,
        },
    )
    .await?;

    log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

    // TODO atomicity with on-chain request
    // ideally store this data (basically the project id + description) on chain too
    // use note prefix + indexer to query

    let saved_request = api
        .submit_withdrawal_request(&WithdrawalRequestInputs {
            project_id: pars.pt.project_id,
            slot_id: pars.pt.slot_id.to_string(),
            amount: algos_str_to_microalgos(&pars.pt.amount)?,
            description: pars.description,
        })
        .await?;

    // log::debug!("Backend saved the request: {:?}", saved_request);

    // TODO return withdrawal list to refresh UI (in the future only most recent x entries)

    Ok(SubmitInitWithdrawalRequestResJs {
        message: "Success, withdrawal success!".to_owned(),
        saved_request: withdrawal_req_to_view_data(
            &saved_request,
            // request was just created so no one has voted yet. TODO refactor formatting
            "0 %".to_owned(),
            false,
        )?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitInitWithdrawalRequestParJs {
    pub pt: SubmitInitWithdrawalRequestPassthroughParJs,
    pub description: String, // this one is not really pt, as it was not passed to the previous step (currently not needed for the on-chain submission)
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitInitWithdrawalRequestPassthroughParJs {
    pub project_id: String,
    pub slot_id: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitInitWithdrawalRequestResJs {
    pub message: String,
    pub saved_request: WithdrawalRequestViewData,
}
