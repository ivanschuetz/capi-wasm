use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use crate::js::investment::submit_harvest::SubmitClaimPassthroughParJs;
use crate::service::drain_if_needed::drain_if_needed_txs;
use anyhow::{Error, Result};
use core::dependencies::algod;
use core::flows::claim::claim::claim;
use core::flows::create_dao::storage::load_dao::load_dao;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_claim, pars: {:?}", pars);
    to_bridge_res(_bridge_bridge_claim(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_bridge_claim(pars: ClaimParJs) -> Result<ClaimResJs> {
    let algod = algod();
    let api = api();
    let funds_asset_id = funds_asset_specs()?.id;
    let capi_deps = capi_deps()?;

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

    let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

    let to_sign_for_claim = claim(&algod, investor_address, dao.app_id, funds_asset_id).await?;

    let mut to_sign = vec![to_sign_for_claim.app_call_tx];

    let maybe_to_sign_for_drain =
        drain_if_needed_txs(&algod, &dao, investor_address, funds_asset_id, &capi_deps).await?;

    // we append drain at the end since it's optional, so the indices of the non optional txs are fixed
    let mut maybe_drain_tx_msg_pack = None;
    let mut maybe_capi_share_tx_msg_pack = None;
    if let Some(to_sign_for_drain) = maybe_to_sign_for_drain {
        to_sign.push(to_sign_for_drain.app_call_tx);
        to_sign.push(to_sign_for_drain.capi_app_call_tx);
        maybe_drain_tx_msg_pack = Some(rmp_serde::to_vec_named(&to_sign_for_drain.drain_tx)?);
        maybe_capi_share_tx_msg_pack =
            Some(rmp_serde::to_vec_named(&to_sign_for_drain.capi_share_tx)?);
    }

    Ok(ClaimResJs {
        to_sign: to_my_algo_txs1(&to_sign).map_err(Error::msg)?,
        pt: SubmitClaimPassthroughParJs {
            maybe_drain_tx_msg_pack,
            maybe_capi_share_tx_msg_pack,
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClaimParJs {
    pub dao_id: String,
    pub amount: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClaimResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitClaimPassthroughParJs,
}
