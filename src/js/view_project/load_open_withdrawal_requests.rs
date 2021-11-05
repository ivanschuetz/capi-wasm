use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
};
use algonaut::algod::v2::Algod;
use anyhow::Result;
use core::{
    flows::create_project::model::Project,
    state::withdrawal_app_state::withdrawal_slot_global_state,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_open_withdrawal_requests(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_open_withdrawal_requests, pars: {:?}", pars);
    to_bridge_res(_bridge_load_open_withdrawal_requests(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_open_withdrawal_requests(
    pars: LoadOpenWithdrawalRequestsParJs,
) -> Result<LoadOpenWithdrawalRequestsResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let open_request_count = open_withdrawal_request_count(&algod, &project)
        .await?
        .to_string();

    Ok(LoadOpenWithdrawalRequestsResJs { open_request_count })
}

async fn open_withdrawal_request_count(algod: &Algod, project: &Project) -> Result<u32> {
    let mut count = 0u32;
    for slot_id in &project.withdrawal_slot_ids {
        let slot_state = withdrawal_slot_global_state(&algod, *slot_id).await?;
        if slot_state.has_active_request() {
            count += 1;
        }
    }
    Ok(count)
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadOpenWithdrawalRequestsParJs {
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadOpenWithdrawalRequestsResJs {
    open_request_count: String,
}
