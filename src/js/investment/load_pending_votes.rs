use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
};
use anyhow::{Error, Result};
use make::withdrawal_app_state::{
    did_vote_local_state_or_err, has_active_withdrawal_request_global_state_or_err,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_pending_votes(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_pending_votes, pars: {:?}", pars);
    to_bridge_res(_bridge_load_pending_votes(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_pending_votes(
    pars: LoadPendingVotesParJs,
) -> Result<LoadPendingVotesResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let account = algod
        .account_information(&pars.investor_address.parse().map_err(Error::msg)?)
        .await?;
    let mut pending_vote_count = 0u32;
    // Withdrawal requests waiting for investor's vote: they're active and investor hasn't voted yet
    for slot_id in project.withdrawal_slot_ids {
        let slot_app = algod.application_information(slot_id).await?;
        if has_active_withdrawal_request_global_state_or_err(&slot_app)? {
            if !did_vote_local_state_or_err(&account.apps_local_state, slot_id)? {
                pending_vote_count += 1;
            }
        }
    }

    log::debug!("User's pending vote count: {}", pending_vote_count);

    Ok(LoadPendingVotesResJs {
        pending_votes: pending_vote_count.to_string(),
    })
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct LoadPendingVotesParJs {
    pub project_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadPendingVotesResJs {
    pending_votes: String,
}
