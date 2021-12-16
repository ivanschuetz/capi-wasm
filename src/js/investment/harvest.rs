use crate::dependencies::environment;
use crate::js::investment::submit_harvest::SubmitHarvestPassthroughParJs;
use crate::service::drain_if_needed::drain_if_needed_txs;
use crate::{
    dependencies::{algod, api},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
};
use algonaut::core::MicroAlgos;
use anyhow::{Error, Result};
use core::flows::harvest::logic::harvest;
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

    let to_sign_for_harvest = harvest(
        &algod,
        investor_address,
        project.central_app_id,
        MicroAlgos(pars.amount.parse()?),
        &project.central_escrow,
    )
    .await?;

    let mut to_sign = vec![
        to_sign_for_harvest.app_call_tx,
        to_sign_for_harvest.pay_fee_tx,
    ];

    let maybe_to_sign_for_drain = drain_if_needed_txs(&algod, &project, investor_address).await?;
    // we append drain at the end since it's optional, so the indices of the non optional txs are fixed
    let mut maybe_drain_tx_msg_pack = None;
    if let Some(to_sign_for_drain) = maybe_to_sign_for_drain {
        to_sign.push(to_sign_for_drain.pay_fee_tx);
        to_sign.push(to_sign_for_drain.app_call_tx);
        maybe_drain_tx_msg_pack = Some(rmp_serde::to_vec_named(&to_sign_for_drain.drain_tx)?);
    }

    Ok(HarvestResJs {
        to_sign: to_my_algo_txs1(&to_sign).map_err(Error::msg)?,
        pt: SubmitHarvestPassthroughParJs {
            maybe_drain_tx_msg_pack,
            harvest_tx_msg_pack: rmp_serde::to_vec_named(&to_sign_for_harvest.harvest_tx)?,
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct HarvestParJs {
    pub project_id: String,
    pub amount: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HarvestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitHarvestPassthroughParJs,
}
