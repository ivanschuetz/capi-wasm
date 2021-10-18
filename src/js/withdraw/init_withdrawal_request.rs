use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
    service::str_to_algos::algos_str_to_microalgos,
};
use algonaut::algod::v2::Algod;
use anyhow::{anyhow, Error, Result};
use make::{
    flows::{create_project::model::Project, withdraw::init_withdrawal::init_withdrawal},
    withdrawal_app_logic::slot_is_free,
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

    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let free_slot = find_free_withdrawal_slot(&algod, &project).await?;

    let to_sign = init_withdrawal(
        &algod,
        &pars.sender.parse().map_err(Error::msg)?,
        algos_str_to_microalgos(&pars.withdrawal_amount)?,
        free_slot,
    )
    .await?;

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
        if slot_is_free(algod, *slot_id).await? {
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
}

#[derive(Debug, Clone, Serialize)]
pub struct InitWithdrawalRequestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitInitWithdrawalRequestPassthroughParJs,
}
