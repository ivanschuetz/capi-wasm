use crate::{dependencies::FundsAssetSpecs, inputs_validation::ValidationError};
use algonaut::core::MicroAlgos;
use anyhow::{anyhow, Result};
use mbase::models::{funds::FundsAmount, share_amount::ShareAmount};
use rust_decimal::{prelude::ToPrimitive, Decimal};

#[allow(dead_code)] // we might use Algo inputs in the future e.g. for fees
pub fn validate_algos_input(s: &str) -> Result<MicroAlgos, ValidationError> {
    validate_algos(s.parse().map_err(|_| ValidationError::NotADecimal)?)
}

pub fn validate_funds_amount_input(
    s: &str,
    asset_specs: &FundsAssetSpecs,
) -> Result<FundsAmount, ValidationError> {
    validate_funds_amount(
        s.parse().map_err(|_| ValidationError::NotADecimal)?,
        asset_specs,
    )
}

pub fn validate_share_count(input: &str) -> Result<ShareAmount> {
    // TODO < available shares (asset count in investing escrow).
    let share_count = input.parse()?;
    if share_count == 0 {
        return Err(anyhow!("Please enter a valid share count"));
    }
    Ok(ShareAmount::new(share_count))
}

#[allow(dead_code)] // we might use Algo inputs in the future e.g. for fees
pub fn microalgos_to_algos_str(micro_algos: MicroAlgos) -> String {
    format!("{:.2}", microalgos_to_algos(micro_algos))
}

pub fn microalgos_to_algos(micro_algos: MicroAlgos) -> Decimal {
    Decimal::from_i128_with_scale(micro_algos.0 as i128, 6).normalize()
}

pub fn base_units_to_display_units_str(
    funds: FundsAmount,
    asset_specs: &FundsAssetSpecs,
) -> String {
    format_display_units(base_units_to_display_units(funds, asset_specs))
}

pub fn format_display_units(display_units: Decimal) -> String {
    format!("{:.2}", display_units)
}

pub fn base_units_to_display_units(funds: FundsAmount, asset_specs: &FundsAssetSpecs) -> Decimal {
    Decimal::from_i128_with_scale(funds.val() as i128, asset_specs.decimals).normalize()
}

fn validate_algos(amount: Decimal) -> Result<MicroAlgos, ValidationError> {
    let amount = amount.normalize();

    if amount.is_sign_negative() || amount.is_zero() {
        return Err(ValidationError::Min {
            min: 0.to_string(),
            actual: amount.to_string(),
        });
    };

    Ok(MicroAlgos(to_base_units(amount, 6)?))
}

fn validate_funds_amount(
    amount: Decimal,
    asset_specs: &FundsAssetSpecs,
) -> Result<FundsAmount, ValidationError> {
    let amount = amount.normalize();

    if amount.is_sign_negative() || amount.is_zero() {
        return Err(ValidationError::Min {
            min: 0.to_string(),
            actual: amount.to_string(),
        });
    };

    Ok(FundsAmount::new(to_base_units(
        amount,
        asset_specs.decimals,
    )?))
}

fn to_base_units(decimal: Decimal, base_10_exp: u32) -> Result<u64, ValidationError> {
    let multiplier = Decimal::from_i128_with_scale(
        10u64.checked_pow(base_10_exp).ok_or_else(|| {
            ValidationError::Unexpected(format!(
                "exp overflow decimal: {}, exp: {}",
                decimal, base_10_exp
            ))
        })? as i128,
        0,
    );

    let base_units = (decimal * multiplier).normalize();
    if base_units.scale() != 0 {
        return Err(ValidationError::TooManyFractionalDigits {
            max: base_10_exp.to_string(),
            actual: decimal.scale().to_string(),
        });
    }

    if base_units > Decimal::from_i128_with_scale(u64::MAX as i128, 0) {
        return Err(ValidationError::Unexpected(format!(
            "Base units: {} overflow, decimal: {}, exp: {}",
            base_units, decimal, base_10_exp
        )));
    }

    base_units.to_u64().ok_or_else(|| {
        ValidationError::Unexpected(format!("Couldn't convert decimal: {} to u64", decimal))
    })
}

pub fn format_short(d: Decimal) -> Result<String> {
    let thousand = 1_000.into();
    let million = 1_000_000.into();
    let billion = 1_000_000_000.into();
    let trillion = 1_000_000_000_000u64.into();

    if d < thousand {
        format_one_fractional_with_suffix(d, "")
    } else if d >= thousand && d < million {
        let r = d.checked_div(thousand).unwrap();
        format_one_fractional_with_suffix(r, "K")
    } else if d >= million && d < billion {
        let r = d.checked_div(million).unwrap();
        format_one_fractional_with_suffix(r, "B")
    } else {
        let r = d.checked_div(trillion).unwrap();
        format_one_fractional_with_suffix(r, "T")
    }
}

fn format_one_fractional_with_suffix(d: Decimal, suffix: &str) -> Result<String> {
    // we want to format amount with x decimals and *skipping trailing zeros*
    // rust currently doesn't have a built in format to skip trailing zeros ({:.N} doesn't)
    // see also https://stackoverflow.com/questions/59506403/how-to-format-a-float-without-trailing-zeros-in-rust
    // so we do this arithmetic: multiplying, rounding and dividing by decimals_multiplier gives us the x decimals,
    // and since we're using Decimal we've to call normalize() to remove the trailing zeros

    let one_fractional_multiplier = 10.into();

    // "part1" is an operand, isolated only for logging
    let part1 = (d.checked_mul(one_fractional_multiplier))
        .ok_or(anyhow!(
            "Error multiplying: {} * {}",
            d,
            one_fractional_multiplier
        ))?
        .round();
    let y = part1
        .checked_div(one_fractional_multiplier)
        .ok_or(anyhow!(
            "Error dividing: {} / {}",
            part1,
            one_fractional_multiplier
        ))?
        .normalize();

    Ok(format!("{}{}", y, suffix))
}
