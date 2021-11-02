use crate::{
    dependencies::{algod, environment},
    js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs},
};
use anyhow::{anyhow, Result};
use core::{
    flows::vote::logic::{submit_vote, VoteSigned},
    withdrawal_app_state::votes_global_state,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_vote(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_vote, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_vote(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_vote(pars: SubmitVoteParJs) -> Result<SubmitVoteResJs> {
    let env = &environment();
    let algod = algod(env);

    if pars.txs.len() != 2 {
        return Err(anyhow!(
            "Unexpected signed vote txs length: {}",
            pars.txs.len()
        ));
    }

    let vote_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
    let validate_vote_count_tx = signed_js_tx_to_signed_tx1(&pars.txs[1])?;

    let withdraw_tx_id = submit_vote(
        &algod,
        &VoteSigned {
            vote_tx,
            validate_vote_count_tx,
        },
    )
    .await?;

    log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

    let slot_app = algod.application_information(pars.slot_id.parse()?).await?;
    let votes =
        votes_global_state(&slot_app).ok_or(anyhow!("No votes in app: {}", pars.slot_id))?;

    Ok(SubmitVoteResJs {
        updated_votes: votes.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitVoteParJs {
    pub project_id: String,
    pub slot_id: String,
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitVoteResJs {
    pub updated_votes: String,
}
