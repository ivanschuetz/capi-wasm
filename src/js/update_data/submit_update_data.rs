use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use anyhow::Result;
use base::dependencies::algod;
use base::flows::update_data::update_data::{submit_update_data, UpdateDaoDataSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_update_dao_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_update_dao_data, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_update_dao_data(parse_bridge_pars(pars)?).await)
}

async fn _bridge_submit_update_dao_data(
    pars: SubmitUpdateAppParJs,
) -> Result<SubmitUpdateAppResJs> {
    let algod = algod();

    let submit_update_res = submit_update_data(
        &algod,
        UpdateDaoDataSigned {
            update: signed_js_tx_to_signed_tx1(&pars.tx)?,
        },
    )
    .await?;

    log::debug!("Submit update dao data res: {:?}", submit_update_res);

    Ok(SubmitUpdateAppResJs {})
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitUpdateAppParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitUpdateAppResJs {}
