use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs,
};
use anyhow::Result;
use base::dependencies::algod;
use base::roadmap::add_roadmap_item::{submit_add_roadmap_item, AddRoadmapItemToSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_add_roadmap_item, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_add_roadmap_item(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_add_roadmap_item(
    pars: SubmitAddRoadmapItemParJs,
) -> Result<SubmitAddRoadmapItemResJs> {
    let algod = algod();

    let add_roadmap_item_signed_tx = signed_js_tx_to_signed_tx1(&pars.tx)?;

    let tx_id = submit_add_roadmap_item(
        &algod,
        &AddRoadmapItemToSigned {
            tx: add_roadmap_item_signed_tx,
        },
    )
    .await?;

    Ok(SubmitAddRoadmapItemResJs { tx_id })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitAddRoadmapItemParJs {
    pub tx: SignedTxFromJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitAddRoadmapItemResJs {
    pub tx_id: String,
}
