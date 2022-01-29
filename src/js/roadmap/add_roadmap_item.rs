use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_tx1};
use algonaut::crypto::HashDigest;
use anyhow::{anyhow, Error, Result};
use core::date_util::timestamp_seconds_to_date;
use core::dependencies::algod;
use core::roadmap::add_roadmap_item::{add_roadmap_item, RoadmapItemInputs};
use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_add_roadmap_item, pars: {:?}", pars);
    to_bridge_res(_bridge_add_roadmap_item(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_add_roadmap_item(pars: AddRoadmapItemParJs) -> Result<AddRoadmapItemResJs> {
    let algod = algod();

    let project_creator = pars.creator_address.parse().map_err(Error::msg)?;
    let project_id = pars.project_id.parse()?;

    let parent_hash = hash_str_option_to_hash_option(pars.parent)?;

    let date = timestamp_seconds_to_date(pars.date.parse()?)?;

    let to_sign = add_roadmap_item(
        &algod,
        &project_creator,
        &RoadmapItemInputs {
            project_id,
            title: pars.title,
            parent: Box::new(parent_hash),
            date,
        },
    )
    .await?;

    Ok(AddRoadmapItemResJs {
        to_sign: to_my_algo_tx1(&to_sign.tx)?,
    })
}

fn hash_str_option_to_hash_option(hash_str: Option<String>) -> Result<Option<HashDigest>> {
    Ok(match &hash_str {
        Some(hash_str) => {
            let bytes = BASE64.decode(hash_str.as_bytes())?;
            Some(HashDigest(bytes.clone().try_into().map_err(|e| {
                anyhow!(
                    "Couldn't convert bytes(len: {:?}): {:?} into hash. e: {:?}",
                    bytes.len(),
                    bytes,
                    e
                )
            })?))
        }
        None => None,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddRoadmapItemParJs {
    pub creator_address: String,
    pub project_id: String,
    pub title: String,
    pub date: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddRoadmapItemResJs {
    pub to_sign: Value,
}
