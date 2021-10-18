use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
};
use anyhow::{anyhow, Error, Result};
use data_encoding::BASE64;
use make::flows::vote::logic::vote;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_vote(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_vote, pars: {:?}", pars);
    to_bridge_res(_bridge_vote(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_vote(pars: VoteParJs) -> Result<VoteResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    // TODO convenience fns for local / global state reading (in make project)

    let voter_account_infos = algod
        .account_information(&pars.voter_address.parse().map_err(Error::msg)?)
        .await?;

    let app_local_vars = voter_account_infos
        .apps_local_state
        .into_iter()
        .find(|ls| ls.id == project.central_app_id)
        .ok_or(anyhow!(
            "No local state for app: {}.",
            project.central_app_id
        ))?
        .key_value;

    let voter_shares_count = app_local_vars
        .iter()
        .find(|kv| kv.key == BASE64.encode(b"Shares").to_owned())
        .map(|kv| kv.value.uint)
        // TODO confirm that not existent local state key means that nothing was harvested yet (0)
        // we currently assume it's the case
        .unwrap_or_else(|| 0);

    let to_sign = vote(
        &algod,
        pars.voter_address.parse().map_err(Error::msg)?,
        project.central_app_id,
        pars.slot_id.parse()?,
        voter_shares_count,
    )
    .await?;

    Ok(VoteResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.vote_tx, to_sign.validate_vote_count_tx])?,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoteParJs {
    pub project_id: String,
    pub slot_id: String,
    pub voter_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VoteResJs {
    pub to_sign: Vec<Value>,
}
