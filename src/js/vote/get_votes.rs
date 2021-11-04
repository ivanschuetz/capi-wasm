// TODO probably file can be deleted - we don't need bridge only to get votes? if not check repeated code with get_votes_percentage in load_requests

use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res},
};
use anyhow::Result;
use core::{
    decimal_util::{AsDecimal, DecimalExt},
    state::withdrawal_app_state::withdrawal_slot_global_state,
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

    let slot_id = pars.slot_id.parse()?;

    let project = api.load_project(&pars.project_id).await?;

    let slot_gs = withdrawal_slot_global_state(&algod, slot_id).await?;

    let percentage = slot_gs.votes.as_decimal() / project.specs.shares.count.as_decimal();
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
