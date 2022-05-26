mod dependencies;
mod inputs_validation;
pub mod js;
mod model;
mod provider;
mod service;
pub mod teal;

use anyhow::{anyhow, Result};
use base::api::image_api::ImageApi;
use mbase::{
    models::{
        hash::GlobalStateHash, share_amount::ShareAmount, shares_percentage::SharesPercentage,
    },
    util::decimal_util::AsDecimal,
};
use rust_decimal::Decimal;

/// Calculates the entitled profit percentage for having a certain amount of shares locked
fn calculate_profit_percentage(
    locked_shares_amount: ShareAmount,
    share_supply: ShareAmount,
    investors_share: SharesPercentage,
) -> Result<Decimal> {
    let perc_of_supply = locked_shares_amount
        .val()
        .as_decimal()
        .checked_div(share_supply.as_decimal())
        .ok_or(anyhow!(
            "Error dividing: {share_count} / {share_supply}".to_owned()
        ))?;

    let perc = perc_of_supply
        .checked_mul(investors_share.value())
        .ok_or(anyhow!(
            "Error multiplying: {perc_of_supply} * {investors_share}".to_owned()
        ))?;

    Ok(perc)
}

pub trait GlobalStateHashExt {
    fn as_api_url(&self, api: &dyn ImageApi) -> String;
}

impl GlobalStateHashExt for GlobalStateHash {
    fn as_api_url(&self, api: &dyn ImageApi) -> String {
        api.image_url(&self.as_api_id())
    }
}
