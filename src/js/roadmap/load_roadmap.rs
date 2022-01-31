use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::js::explorer_links::explorer_tx_id_link_env;
use anyhow::{anyhow, Error, Result};
use chrono::Datelike;
use core::roadmap::get_roadmap::get_roadmap;
use core::{dependencies::indexer, roadmap::get_roadmap::SavedRoadmapItem};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Debug};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_roadmap(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_roadmap, pars: {:?}", pars);
    to_bridge_res(_bridge_load_roadmap(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_load_roadmap(pars: GetRoadmapParJs) -> Result<GetRoadmapResJs> {
    let indexer = indexer();

    let project_creator = pars.creator_address.parse().map_err(Error::msg)?;
    let project_id = pars.project_id.parse()?;

    let roadmap = get_roadmap(&indexer, &project_creator, &project_id).await?;
    let mut items = roadmap.items;
    // sort ascendingly by date
    items.sort_by(|i1, i2| i1.date.cmp(&i2.date));

    let grouped = group_by_time_range(items)?;
    let all_and_flat_items = to_flat_roadmap_items(grouped);

    Ok(GetRoadmapResJs {
        items: to_js_items(all_and_flat_items),
    })
}

/// Assumes that items is sorted ascendingly.
/// The returned keys as well as the items in the values are sorted ascendingly as well.
fn group_by_time_range(
    items: Vec<SavedRoadmapItem>,
) -> Result<BTreeMap<YearQuarter, Vec<SavedRoadmapItem>>> {
    let mut map = BTreeMap::new();

    for item in items {
        let quarter = to_quarter(item.date.month())?;
        log::debug!(
            "date: {}, month: {}, quarter: {:?}",
            item.date,
            item.date.month(),
            quarter
        );
        let year = item.date.year();
        let year_quarter = YearQuarter { quarter, year };

        map.entry(year_quarter).or_insert(vec![]).push(item);
    }

    Ok(map)
}

// Note 1-indexed month
fn to_quarter(month: u32) -> Result<Quarter> {
    match month {
        1..=3 => Ok(Quarter::Q1),
        4..=6 => Ok(Quarter::Q2),
        7..=9 => Ok(Quarter::Q3),
        10..=12 => Ok(Quarter::Q4),
        _ => Err(anyhow!("Invalid month number: {month}")),
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct YearQuarter {
    // NOTE: don't change field order (PartialOrd macro)
    year: i32,
    quarter: Quarter,
}

impl ToString for YearQuarter {
    fn to_string(&self) -> String {
        format!("{} {}", self.quarter.to_string(), self.year)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Quarter {
    // NOTE: don't change field order (PartialOrd macro)
    Q1,
    Q2,
    Q3,
    Q4,
}

impl ToString for Quarter {
    fn to_string(&self) -> String {
        format!("{self:?}")
    }
}

fn to_flat_roadmap_items(
    grouped_items: BTreeMap<YearQuarter, Vec<SavedRoadmapItem>>,
) -> Vec<RoadmapItemView> {
    let mut flat_items = vec![];

    for year_quarter in grouped_items.keys() {
        let items = grouped_items[year_quarter].clone();
        for item in items {
            flat_items.push(RoadmapItemView::Item(item));
        }
        // header at the end (for UI)
        flat_items.push(RoadmapItemView::Header(year_quarter.to_owned()));
    }

    flat_items
}

fn to_js_items(items: Vec<RoadmapItemView>) -> Vec<RoadmapItemJs> {
    items.into_iter().map(|i| to_js_item(i)).collect()
}

fn to_js_item(item: RoadmapItemView) -> RoadmapItemJs {
    match item {
        RoadmapItemView::Item(item) => RoadmapItemJs {
            item_type: "item".to_owned(),
            tx_id: Some(item.tx_id.to_string()),
            tx_link: Some(explorer_tx_id_link_env(&item.tx_id)),
            date: Some(item.date.timestamp().to_string()),
            text: item.title,
        },
        RoadmapItemView::Header(year_quarter) => RoadmapItemJs {
            item_type: "header".to_owned(),
            tx_id: None,
            tx_link: None,
            date: None,
            text: year_quarter.to_string(),
        },
    }
}

#[derive(Debug, Clone)]
enum RoadmapItemView {
    Item(SavedRoadmapItem),
    Header(YearQuarter),
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
    pub item_type: String,       // "item" | "header"
    pub tx_id: Option<String>,   // set if type == "item"
    pub tx_link: Option<String>, // set if type == "item"
    pub date: Option<String>,    // set if type == "item"
    pub text: String,
}
