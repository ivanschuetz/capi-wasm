use std::convert::TryInto;
use std::str::FromStr;

use crate::calculate_profit_percentage;
use crate::provider::calculate_total_price::{
    CalculateTotalPriceParJs, CalculateTotalPriceProvider, CalculateTotalPriceResJs,
};
use crate::provider::investment_provider::CalcPriceAndPercSpecs;
use crate::service::str_to_algos::{
    base_units_to_display_units_str, validate_funds_amount_input, validate_share_count,
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
    async fn get(&self, pars: CalculateTotalPriceParJs) -> Result<CalculateTotalPriceResJs> {
        let specs: CalcPriceAndPercSpecs = rmp_serde::from_slice(&pars.share_specs_msg_pack)?;

        let validated_price = validate_funds_amount_input(&pars.share_price, &specs.funds_specs)?;
        let validated_share_amount = validate_share_count(&pars.shares_amount)?;

        let available_shares = ShareAmount::new(pars.available_shares.parse().map_err(Error::msg)?);
        let share_supply = ShareAmount::new(pars.share_supply.parse().map_err(Error::msg)?);
        let investors_share_dec = Decimal::from_str(&pars.investors_share)?;
        let investors_share = investors_share_dec.try_into()?;

        if validated_share_amount > available_shares {
            return Err(anyhow!(
                "Share amount ({validated_share_amount}) must be <= available shares ({available_shares})"
            ));
        }

        let total_price = FundsAmount::new(
            validated_share_amount
                .val()
                .checked_mul(validated_price.val())
                .ok_or(anyhow!(
                    "Overflow multiplying: {validated_share_amount} * {validated_price}"
                ))?,
        );

        let profit_percentage =
            calculate_profit_percentage(validated_share_amount, share_supply, investors_share)?;

        let total_price_display = base_units_to_display_units_str(total_price, &specs.funds_specs);

        Ok(CalculateTotalPriceResJs {
            total_price: total_price_display,
            profit_percentage: profit_percentage.format_percentage(),
        })
    }
}
