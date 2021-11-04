use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
    service::str_to_algos::validate_algos_input,
};
use algonaut::{algod::v2::Algod, core::MicroAlgos};
use anyhow::{anyhow, Error, Result};
use core::{
    flows::{create_project::model::Project, withdraw::init_withdrawal::init_withdrawal},
    state::withdrawal_app_state::withdrawal_slot_global_state,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

use super::submit_init_withdrawal_request::SubmitInitWithdrawalRequestPassthroughParJs;

#[wasm_bindgen]
pub async fn bridge_init_withdrawal_request(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_init_withdrawal_request, pars: {:?}", pars);
    to_bridge_res(_bridge_init_withdrawal_request(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_init_withdrawal_request(
    pars: InitWithdrawalRequestParJs,
) -> Result<InitWithdrawalRequestResJs> {
    log::debug!("_bridge_init_withdrawal_request, pars: {:?}", pars);

    let amount = validate_withdrawal_amount(&pars.withdrawal_amount)?;
    validate_withdrawal_description(&pars.description)?; // not used in this step, only validation
    let sender = pars.sender.parse().map_err(Error::msg)?;

    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let free_slot = find_free_withdrawal_slot(&algod, &project).await?;

    let to_sign = init_withdrawal(&algod, &sender, amount, free_slot).await?;

    Ok(InitWithdrawalRequestResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.init_withdrawal_slot_app_call_tx])?,
        pt: SubmitInitWithdrawalRequestPassthroughParJs {
            project_id: pars.project_id,
            slot_id: free_slot.to_string(),
            amount: pars.withdrawal_amount,
        },
    })
}

async fn find_free_withdrawal_slot(algod: &Algod, project: &Project) -> Result<u64> {
    for slot_id in &project.withdrawal_slot_ids {
        let slot_gs = withdrawal_slot_global_state(&algod, *slot_id).await?;
        if slot_gs.is_free() {
            return Ok(*slot_id);
        }
    }
    Err(anyhow!("No free withdrawal slots."))
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitWithdrawalRequestParJs {
    pub project_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String, // for validation
}

#[derive(Debug, Clone, Serialize)]
pub struct InitWithdrawalRequestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitInitWithdrawalRequestPassthroughParJs,
}

pub fn validate_withdrawal_amount(input: &str) -> Result<MicroAlgos> {
    // Note that we can init a request for more than currently in the funds
    // funds can increase or decrease after creating the request, so there's no point in constraining the amount here.
    validate_algos_input(input)
}

pub fn validate_withdrawal_description(input: &str) -> Result<String> {
    let description = input.trim();

    let max_length = 500;

    let description_len = description.len();
    if description_len > max_length {
        return Err(anyhow!(
            "Request description must not have more than {} characters. Current: {}",
            max_length,
            description_len
        ));
    }

    Ok(description.to_owned())
}
