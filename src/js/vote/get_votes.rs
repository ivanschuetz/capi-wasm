// TODO probably file can be deleted - we don't need bridge only to get votes? if not check repeated code with get_votes_percentage in load_requests

use crate::{dependencies::{algod, api, environment}, js::{
        common::{parse_bridge_pars, to_bridge_res},
        vote::common::asset_count,
    }};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_get_votes(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_get_votes, pars: {:?}", pars);
    to_bridge_res(_bridge_get_votes(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_get_votes(pars: GetVotesParJs) -> Result<GetVotesResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let vote_in_count = asset_count(
        &algod,
        project.votein_escrow.address,
        project.votes_asset_id,
    )
    .await?;

    // TODO Decimal
    let percentage = vote_in_count as f64 / project.specs.shares.count as f64;
    Ok(GetVotesResJs {
        votes_percentage: format!("{} %", percentage * 100 as f64),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetVotesParJs {
    pub project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetVotesResJs {
    pub votes_percentage: String,
}
