use crate::js::{
    common::{parse_bridge_pars, to_bridge_res},
    explorer_links::explorer_address_link_env,
};
use algonaut::core::Address;
use anyhow::{anyhow, Result};
use core::{
    decimal_util::DecimalExt, dependencies::indexer,
    queries::shares_distribution::shares_holders_distribution,
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_shares_distribution(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_shares_distribution, pars: {:?}", pars);
    to_bridge_res(_bridge_shares_distribution(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_shares_distribution(
    pars: SharedDistributionParJs,
) -> Result<SharedDistributionResJs> {
    let indexer = indexer();

    let asset_id = pars.asset_id.parse()?;
    let asset_supply = pars.asset_supply.parse()?;

    let holders = shares_holders_distribution(&indexer, asset_id, asset_supply).await?;

    let mut holders_js = vec![];
    for h in holders {
        holders_js.push(ShareHoldingPercentageJs {
            address: h.address.to_string(),
            short_address: shorten_address(&h.address)?,
            address_browser_link: explorer_address_link_env(&h.address),
            amount: h.amount.to_string(),
            percentage_formatted: h.percentage.format_percentage(),
            percentage_number: h.percentage.to_string(),
        });
    }

    Ok(SharedDistributionResJs {
        holders: holders_js,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SharedDistributionParJs {
    pub asset_id: String,
    /// optimization to not have to fetch the asset: the asset specs are in the project, which the frontend has to fetch first (to get the asset id)
    pub asset_supply: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShareHoldingPercentageJs {
    pub address: String,
    pub short_address: String,
    pub address_browser_link: String,
    pub amount: String,
    pub percentage_formatted: String,
    pub percentage_number: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SharedDistributionResJs {
    pub holders: Vec<ShareHoldingPercentageJs>,
}

fn shorten_address(address: &Address) -> Result<String> {
    let address_str = address.to_string();

    let len = address_str.len();

    if len < 6 {
        return Err(anyhow!("Invalid address (too short): {address}"));
    }

    Ok(format!(
        "{}...{}",
        address_str[0..3].to_owned(),
        address_str[len - 3..len].to_owned()
    ))
}
