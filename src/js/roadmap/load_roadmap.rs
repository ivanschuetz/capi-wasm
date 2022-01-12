use crate::js::{
    common::{parse_bridge_pars, to_bridge_res},
    explorer_links::explorer_tx_id_link_env,
};
use anyhow::{Error, Result};
use core::roadmap::get_roadmap::get_roadmap;
use core::{dependencies::indexer, flows::create_project::storage::load_project::ProjectId};
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
    let project_id = ProjectId(pars.project_id);

    let roadmap = get_roadmap(&indexer, &project_creator, &project_id).await?;

    Ok(GetRoadmapResJs {
        items: roadmap
            .items
            .into_iter()
            .map(|i| RoadmapItemJs {
                tx_id: i.tx_id.clone(),
                tx_link: explorer_tx_id_link_env(&i.tx_id),
                project_id: i.project_id.0,
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
    project_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRoadmapResJs {
    pub items: Vec<RoadmapItemJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapItemJs {
    pub tx_id: String,
    pub tx_link: String,
    pub project_id: String,
    pub title: String,
    pub parent: Option<String>,
    pub hash: String,
}
