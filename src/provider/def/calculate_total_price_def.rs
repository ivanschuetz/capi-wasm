use std::convert::TryInto;
use std::str::FromStr;

use crate::calculate_profit_percentage;
use crate::dependencies::funds_asset_specs;
use crate::error::FrError;
use crate::provider::calculate_total_price::{
    CalculateMaxFundsParJs, CalculateMaxFundsResJs, CalculateTotalPriceParJs,
    CalculateTotalPriceProvider, CalculateTotalPriceResJs,
};
use crate::service::number_formats::{
    base_units_to_display_units_readable, validate_funds_amount_input,
    validate_share_amount_positive, validate_share_amount_positive_or_0,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use mbase::models::funds::FundsAmount;
use mbase::models::share_amount::ShareAmount;
use mbase::util::decimal_util::DecimalExt;
use rust_decimal::Decimal;

pub struct CalculateTotalPriceDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CalculateTotalPriceProvider for CalculateTotalPriceDef {
    /// total price calculation for "buy shares" (share input * dao share price)
    async fn get(
        &self,
        pars: CalculateTotalPriceParJs,
    ) -> Result<CalculateTotalPriceResJs, FrError> {
        let funds_asset_specs = funds_asset_specs()?;

        let validated_price = validate_funds_amount_input(&pars.share_price, &funds_asset_specs)?;
        let validated_share_amount = validate_share_amount_positive(&pars.shares_amount)?;

        let available_shares = ShareAmount::new(pars.available_shares.parse().map_err(Error::msg)?);
        let share_supply = ShareAmount::new(pars.share_supply.parse().map_err(Error::msg)?);
        let locked_shares = match pars.locked_shares {
            Some(s) => Some(validate_share_amount_positive_or_0(&s)?),
            None => None,
        };
        let investors_share_dec = Decimal::from_str(&pars.investors_share).map_err(Error::msg)?;
        let investors_share = investors_share_dec.try_into()?;

        if validated_share_amount > available_shares {
            return Err(anyhow!(
                "Share amount ({validated_share_amount}) must be <= available shares ({available_shares})"
            ).into());
        }

        let total_price = FundsAmount::new(
            validated_share_amount
                .val()
                .checked_mul(validated_price.val())
                .ok_or(anyhow!(
                    "Overflow multiplying: {validated_share_amount} * {validated_price}"
                ))?,
        );

        // to calculate total dividend if the user buys the entered share amount:
        // entered share amount + currently locked shares
        // (remember: bought shares are automatically locked)
        let total_shares_to_calculate_dividend = ShareAmount::new(
            validated_share_amount.val() + locked_shares.map(|s| s.val()).unwrap_or(0),
        );

        let profit_percentage = calculate_profit_percentage(
            total_shares_to_calculate_dividend,
            share_supply,
            investors_share,
        )?;

        let total_price_display =
            base_units_to_display_units_readable(total_price, &funds_asset_specs)?;

        Ok(CalculateTotalPriceResJs {
            total_price: total_price_display,
            total_price_number: total_price.val().to_string(),
            profit_percentage: profit_percentage.format_percentage(),
        })
    }

    /// total price calculation for create dao form (share input * price input)
    async fn max_funds(
        &self,
        pars: CalculateMaxFundsParJs,
    ) -> Result<CalculateMaxFundsResJs, FrError> {
        let funds_asset_specs = funds_asset_specs()?;

        let validated_price = validate_funds_amount_input(&pars.share_price, &funds_asset_specs)?;
        let validated_share_amount = validate_share_amount_positive(&pars.shares_amount)?;

        let total_price = FundsAmount::new(
            validated_share_amount
                .val()
                .checked_mul(validated_price.val())
                .ok_or(anyhow!(
                    "Overflow multiplying: {validated_share_amount} * {validated_price}"
                ))?,
        );

        let total_price_display =
            base_units_to_display_units_readable(total_price, &funds_asset_specs)?;

        Ok(CalculateMaxFundsResJs {
            total_price: total_price_display,
            total_price_number: total_price.val().to_string(),
        })
    }
}
