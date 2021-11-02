// TODO probably file can be deleted - we don't need bridge only to get votes? if not check repeated code with get_votes_percentage in load_requests

use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
};
use anyhow::{anyhow, Result};
use core::{
    decimal_util::{AsDecimal, DecimalExt},
    withdrawal_app_state::votes_global_state,
};
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

    let slot_app = algod.application_information(pars.slot_id.parse()?).await?;
    let votes =
        votes_global_state(&slot_app).ok_or(anyhow!("No votes in app: {}", pars.slot_id))?;

    let percentage = votes.as_decimal() / project.specs.shares.count.as_decimal();
    Ok(GetVotesResJs {
        votes_percentage: percentage.format_percentage(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetVotesParJs {
    pub project_id: String,
    pub slot_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetVotesResJs {
    pub votes_percentage: String,
}
