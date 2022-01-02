use crate::js::common::{parse_bridge_pars, to_bridge_res};
use anyhow::{Error, Result};
use core::dependencies::indexer;
use core::roadmap::get_roadmap::get_roadmap;
use data_encoding::BASE64;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_roadmap(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_roadmap, pars: {:?}", pars);
    to_bridge_res(_bridge_load_roadmap(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_roadmap(pars: GetRoadmapParJs) -> Result<GetRoadmapResJs> {
    let indexer = indexer();

    let project_creator = pars.creator_address.parse().map_err(Error::msg)?;
    let project_uuid = pars.project_uuid.parse()?;

    let roadmap = get_roadmap(&indexer, &project_creator, &project_uuid).await?;

    Ok(GetRoadmapResJs {
        items: roadmap
            .items
            .into_iter()
            .map(|i| RoadmapItemJs {
                project_uuid: i.project_uuid.to_string(),
                title: i.title.clone(),
                parent: i.parent.map(|h| BASE64.encode(&h.0)),
                hash: BASE64.encode(&i.hash.0),
            })
            .collect(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRoadmapParJs {
    creator_address: String,
    project_uuid: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRoadmapResJs {
    pub items: Vec<RoadmapItemJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapItemJs {
    pub project_uuid: String,
    pub title: String,
    pub parent: Option<String>,
    pub hash: String,
}
