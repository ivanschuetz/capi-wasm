use crate::{
    dependencies::{algod, api, environment},
    js::{
        common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs},
        vote::common::asset_count,
    },
};
use anyhow::{anyhow, Result};
use make::flows::vote::logic::{submit_vote, VoteSigned};
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
    let api = api(env);

    if pars.txs.len() != 1 {
        return Err(anyhow!(
            "Unexpected signed vote txs length: {}",
            pars.txs.len()
        ));
    }

    let validate_investor_vote_count_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;

    let withdraw_tx_id = submit_vote(
        &algod,
        &VoteSigned {
            validate_investor_vote_count_tx,
            xfer_tx: rmp_serde::from_slice(&pars.pt.vote_xfer_tx_msg_pack)?,
        },
    )
    .await?;

    log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

    // fetch updated votes to update UI
    let project = api.load_project(&pars.project_id).await?;
    let vote_in_count = asset_count(
        &algod,
        project.votein_escrow.address,
        project.votes_asset_id,
    )
    .await?;

    Ok(SubmitVoteResJs {
        updated_votes: vote_in_count.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitVoteParJs {
    pub project_id: String,
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitVotePassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitVotePassthroughParJs {
    pub vote_xfer_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitVoteResJs {
    pub updated_votes: String,
}
