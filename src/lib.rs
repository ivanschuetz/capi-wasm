#![feature(async_closure)]

mod dependencies;
pub mod error;
mod inputs_validation;
pub mod js;
mod model;
pub mod provider;
mod service;

use anyhow::{anyhow, Result};
use mbase::{
    models::{share_amount::ShareAmount, shares_percentage::SharesPercentage},
    util::decimal_util::AsDecimal,
};
use rust_decimal::Decimal;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
