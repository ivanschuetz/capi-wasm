use crate::js::{
    common::{parse_bridge_pars, to_bridge_res},
    explorer_links::explorer_address_link_env,
};
use algonaut::core::Address;
use anyhow::{anyhow, Error, Result};
use core::{
    decimal_util::{AsDecimal, DecimalExt},
    dependencies::{algod, indexer},
    queries::shares_distribution::{shares_holders_distribution, ShareHoldingPercentage},
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
    let algod = algod();
    let indexer = indexer();

    let asset_id = pars.asset_id.parse()?;
    let share_supply = pars.share_supply.parse()?;
    let app_id = pars.app_id.parse()?;
    let investing_escrow = pars.investing_escrow_address.parse().map_err(Error::msg)?;
    let locking_escrow = pars.locking_escrow_address.parse().map_err(Error::msg)?;

    let holders = shares_holders_distribution(
        &algod,
        &indexer,
        asset_id,
        app_id,
        share_supply,
        &investing_escrow,
        &locking_escrow,
    )
    .await?;

    let mut holders_js = vec![];
    for h in &holders {
        holders_js.push(ShareHoldingPercentageJs {
            address: h.address.to_string(),
            label: shorten_address(&h.address)?,
            address_browser_link: explorer_address_link_env(&h.address),
            amount: h.amount.to_string(),
            percentage_formatted: h.percentage.format_percentage(),
            percentage_number: h.percentage.to_string(),
            type_: "holder".to_owned(),
        });
    }

    holders_js.push(not_owned_shares_holdings(&holders, share_supply)?);

    Ok(SharedDistributionResJs {
        holders: holders_js,
    })
}

fn not_owned_shares_holdings(
    holders: &[ShareHoldingPercentage],
    supply: u64,
) -> Result<ShareHoldingPercentageJs> {
    let total_holders_amount: u64 = holders.into_iter().map(|h| h.amount.val()).sum();

    let not_owned_amount: u64 = supply - total_holders_amount;
    let not_owned_percentage = not_owned_amount
        .as_decimal()
        .checked_div(supply.as_decimal())
        .ok_or_else(|| {
            anyhow!("not_owned_amount: {not_owned_amount:?} / supply: {supply:?} failed")
        })?;

    Ok(ShareHoldingPercentageJs {
        address: "".to_owned(),
        label: "Not owned".to_owned(),
        address_browser_link: "".to_owned(),
        amount: not_owned_amount.to_string(),
        percentage_formatted: not_owned_percentage.format_percentage(),
        percentage_number: not_owned_percentage.to_string(),
        type_: "not_owned".to_owned(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SharedDistributionParJs {
    pub asset_id: String,
    /// optimization to not have to fetch the asset: the asset specs are in the dao, which the frontend has to fetch first (to get the asset id)
    pub share_supply: String,

    pub app_id: String,
    pub investing_escrow_address: String,
    pub locking_escrow_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShareHoldingPercentageJs {
    pub address: String,
    pub label: String,
    pub address_browser_link: String,
    pub amount: String,
    pub percentage_formatted: String,
    pub percentage_number: String,
    pub type_: String,
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
