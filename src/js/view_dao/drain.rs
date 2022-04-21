use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use anyhow::{Error, Result};
use base::dependencies::algod;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::drain::drain::fetch_drain_amount_and_drain;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

use super::submit_drain::SubmitDrainPassthroughParJs;

#[wasm_bindgen]
pub async fn bridge_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_drain, pars: {:?}", pars);
    to_bridge_res(_bridge_drain(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_drain(pars: DrainParJs) -> Result<DrainResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

    let to_sign = fetch_drain_amount_and_drain(
        &algod,
        &pars.drainer_address.parse().map_err(Error::msg)?,
        dao.app_id,
        funds_asset_specs()?.id,
        &capi_deps,
        &dao.customer_escrow.account,
    )
    .await?;

    Ok(DrainResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.app_call_tx, to_sign.capi_app_call_tx])?,
        pt: SubmitDrainPassthroughParJs {
            drain_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.drain_tx)?,
            capi_share_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.capi_share_tx)?,
            dao_id: dao_id.to_string(),
        },
    })
}

// TODO this can be optimized passing the already loaded dao from JS
// to not load the dao again
// (we'd have to use the complete dao instance - drain needs lsig)
#[derive(Debug, Clone, Deserialize)]
pub struct DrainParJs {
    pub dao_id: String,
    pub drainer_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DrainResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitDrainPassthroughParJs,
}
