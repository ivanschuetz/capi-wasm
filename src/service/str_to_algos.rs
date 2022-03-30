use crate::{dependencies::FundsAssetSpecs, inputs_validation::ValidationError};
use algonaut::core::MicroAlgos;
use anyhow::Result;
use core::funds::FundsAmount;
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
    format!("{:.2}", base_units_to_display_units(funds, asset_specs))
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
