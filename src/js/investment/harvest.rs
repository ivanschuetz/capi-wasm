// TODO probably file can be deleted - we don't need bridge only to get votes? if not check repeated code with get_votes_percentage in load_requests

use crate::dependencies::environment;
use crate::js::investment::submit_harvest::SubmitHarvestPassthroughParJs;
use crate::service::app_state::{
    investor_can_harvest_amount_from_local_vars, local_vars, owned_shares_count_from_local_vars,
};
use crate::{
    dependencies::{algod, api},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
};
use anyhow::{Error, Result};
use make::flows::harvest::logic::harvest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_harvest(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_harvest, pars: {:?}", pars);
    to_bridge_res(_bridge_bridge_harvest(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_bridge_harvest(pars: HarvestParJs) -> Result<HarvestResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

    let app_local_vars = local_vars(
        &algod,
        &pars.investor_address.parse().map_err(Error::msg)?,
        project.central_app_id,
    )
    .await?;
    let investor_shares_count = owned_shares_count_from_local_vars(&app_local_vars).await?;
    let amount = investor_can_harvest_amount_from_local_vars(
        &algod,
        project.central_app_id,
        &app_local_vars,
        investor_shares_count,
        project.specs.shares.count,
    )
    .await?;

    let to_sign = harvest(
        &algod,
        &investor_address,
        project.central_app_id,
        amount,
        &project.central_escrow,
    )
    .await?;

    Ok(HarvestResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.app_call_tx, to_sign.pay_fee_tx])?,
        pt: SubmitHarvestPassthroughParJs {
            harvest_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.harvest_tx)?,
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct HarvestParJs {
    pub project_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HarvestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitHarvestPassthroughParJs,
}
