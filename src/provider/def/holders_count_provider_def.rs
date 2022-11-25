use std::cmp::Ordering;

use crate::{
    provider::holders_count_provider::{
        HoldersChangeParJs, HoldersChangeResJs, HoldersCountParJs, HoldersCountProvider,
        HoldersCountResJs,
    },
    service::storage::{storage_get, storage_set},
};
use algonaut::core::to_app_address;
use anyhow::Result;
use async_trait::async_trait;
use base::queries::shares_distribution::holders_count;
use chrono::{DateTime, Duration, Utc};
use mbase::{dependencies::indexer, models::dao_app_id::DaoAppId};
use serde::{Deserialize, Serialize};

pub struct HoldersCountProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl HoldersCountProvider for HoldersCountProviderDef {
    async fn get(&self, pars: HoldersCountParJs) -> Result<HoldersCountResJs> {
        let indexer = indexer();

        let asset_id = pars.asset_id.parse()?;
        let app_id: DaoAppId = pars.app_id.parse()?;
        let app_address = to_app_address(app_id.0);

        let holders_count = holders_count(&indexer, asset_id, &app_address).await?;

        Ok(HoldersCountResJs {
            count: holders_count.to_string(),
        })
    }

    async fn change(&self, pars: HoldersChangeParJs) -> Result<HoldersChangeResJs> {
        let indexer = indexer();

        let asset_id = pars.asset_id.parse()?;
        let app_id: DaoAppId = pars.app_id.parse()?;
        let app_address = to_app_address(app_id.0);

        let current_holders_count = holders_count(&indexer, asset_id, &app_address).await?;

        let storage_key = app_id.to_string();

        let last_counts_opt: Option<LastHoldersCounts> = storage_get(&storage_key)?;

        let now = Utc::now();

        let duration = Duration::days(1);

        match last_counts_opt {
            Some(last_counts) => {
                // compare with the most recent count >= duration ago
                let change = if let Some(count) =
                    count_more_than_duration_ago(&last_counts, duration, now)
                {
                    let change_str = match current_holders_count.cmp(&count.count) {
                        Ordering::Less => "down",
                        Ordering::Equal => "eq",
                        Ordering::Greater => "up",
                    };

                    HoldersChangeResJs {
                        change: change_str.to_owned(),
                    }
                } else {
                    // no count to compare with (far enough in the past) - so change can't be determined
                    HoldersChangeResJs {
                        change: "unknown".to_owned(),
                    }
                };

                save_new_count_and_clamp(&last_counts, now, current_holders_count, app_id)?;

                log::debug!("Holders change: {change:?}");
                Ok(change)
            }
            None => {
                // nothing saved yet - just save new count
                storage_set(
                    &app_id.to_string(),
                    &LastHoldersCounts {
                        counts: vec![LastHoldersCount {
                            date: now,
                            count: current_holders_count,
                        }],
                    },
                )?;
                Ok(HoldersChangeResJs {
                    change: "unknown".to_owned(),
                })
            }
        }
    }
}

// saves new count for dao at the end and removes old elements if the vector has more than 2 elements
// the reason we don't need more than 2 elements is that we don't save unless x duration passed since the last entry
// so saved entries are spaced by a duration >= x
// and since we use this to look for "most recent entry with x duration ago", it will be either the last one, or the one before.
fn save_new_count_and_clamp(
    last_counts: &LastHoldersCounts,
    now: DateTime<Utc>,
    current_holders_count: usize,
    app_id: DaoAppId,
) -> Result<()> {
    if let Some(last_count) = last_counts.counts.last() {
        // save a new entry if more than duration since the last entry transcurred
        if now - last_count.date >= Duration::days(1) {
            let mut last_counts = last_counts.clone();
            last_counts.counts.push(LastHoldersCount {
                date: now,
                count: current_holders_count,
            });

            // remove leading entry if there are more than 2 elements
            if last_counts.counts.len() > 2 {
                last_counts.counts.remove(0);
            }

            // save updated counts
            storage_set(&app_id.to_string(), &last_counts)?;
        }
    } else {
        // we expect either no counts stored or counts with elements
        // holders count change icon is unimportant - ignore errors
        log::error!("Invalid state (ignored): counts in storage is empty");
    };

    Ok(())
}

// searches from last to first for a count with saved date >= duration
// assumes that the counts are sorted ascendingly by date
fn count_more_than_duration_ago(
    counts: &LastHoldersCounts,
    duration: Duration,
    now: DateTime<Utc>,
) -> Option<LastHoldersCount> {
    let mut counts = counts.counts.clone();
    counts.reverse();
    counts
        .into_iter()
        .find(|count| now - count.date >= duration)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LastHoldersCount {
    date: DateTime<Utc>,
    count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LastHoldersCounts {
    counts: Vec<LastHoldersCount>,
}
