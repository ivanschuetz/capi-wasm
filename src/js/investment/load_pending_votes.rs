use core::{
    flows::create_project::model::Project,
    state::withdrawal_app_state::{
        withdrawal_slot_global_state, withdrawal_slot_voter_state_with_account,
    },
};

use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
};
use algonaut::{algod::v2::Algod, core::Address};
use anyhow::{Error, Result};
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

    let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

    let pending_votes = pending_vote_slot_count(&algod, &investor_address, &project)
        .await?
        .to_string();

    log::debug!("User's pending vote count: {}", pending_votes);

    Ok(LoadPendingVotesResJs { pending_votes })
}

/// Count of slots with an active withdrawal request and for which the voter hasn't voted yet.
async fn pending_vote_slot_count(
    algod: &Algod,
    investor: &Address,
    project: &Project,
) -> Result<u32> {
    let account = algod.account_information(investor).await?;
    let mut pending_vote_count = 0u32;

    for slot_id in &project.withdrawal_slot_ids {
        let slot_state = withdrawal_slot_global_state(&algod, *slot_id).await?;
        if slot_state.has_active_request() {
            let voter_state = withdrawal_slot_voter_state_with_account(&account, *slot_id)?;
            if !voter_state.did_vote_in_current_round() {
                pending_vote_count += 1;
            }
        }
    }

    Ok(pending_vote_count)
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
