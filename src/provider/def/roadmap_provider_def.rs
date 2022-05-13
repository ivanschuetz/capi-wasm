use crate::js::explorer_links::explorer_tx_id_link_env;
use crate::provider::roadmap_provider::{
    GetRoadmapParJs, GetRoadmapResJs, RoadmapItemJs, RoadmapProvider,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::roadmap::get_roadmap::{get_roadmap, SavedRoadmapItem};
use chrono::Datelike;
use mbase::dependencies::indexer;
use std::collections::BTreeMap;

pub struct RoadmapProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RoadmapProvider for RoadmapProviderDef {
    async fn get(&self, pars: GetRoadmapParJs) -> Result<GetRoadmapResJs> {
        let indexer = indexer();

        let dao_creator = pars.creator_address.parse().map_err(Error::msg)?;
        let dao_id = pars.dao_id.parse()?;

        let roadmap = get_roadmap(&indexer, &dao_creator, dao_id).await?;
        let mut items = roadmap.items;
        // sort ascendingly by date
        items.sort_by(|i1, i2| i1.date.cmp(&i2.date));

        let grouped = group_by_time_range(items)?;
        let all_and_flat_items = to_flat_roadmap_items(grouped);

        Ok(GetRoadmapResJs {
            items: to_js_items(all_and_flat_items),
        })
    }
}

#[derive(Debug, Clone)]
enum RoadmapItemView {
    Item(SavedRoadmapItem),
    Header(YearQuarter),
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

        map.entry(year_quarter).or_insert_with(Vec::new).push(item);
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
    items.into_iter().map(to_js_item).collect()
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
