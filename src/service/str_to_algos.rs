use core::funds::FundsAmount;

use algonaut::core::MicroAlgos;
use anyhow::{anyhow, Result};
use rust_decimal::{prelude::ToPrimitive, Decimal};

use crate::dependencies::FundsAssetSpecs;

#[allow(dead_code)] // we might use Algo inputs in the future e.g. for fees
pub fn validate_algos_input(s: &str) -> Result<MicroAlgos> {
    validate_algos(s.parse()?)
}

pub fn validate_funds_amount_input(s: &str, asset_specs: &FundsAssetSpecs) -> Result<FundsAmount> {
    validate_funds_amount(s.parse()?, asset_specs)
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
    Decimal::from_i128_with_scale(funds.0 as i128, asset_specs.decimals).normalize()
}

fn validate_algos(amount: Decimal) -> Result<MicroAlgos> {
    let amount = amount.normalize();

    if amount.is_sign_negative() || amount.is_zero() {
        return Err(anyhow!("{} amount must be positive (>0)", amount));
    };

    Ok(MicroAlgos(to_base_units(amount, 6)?))
}

fn validate_funds_amount(amount: Decimal, asset_specs: &FundsAssetSpecs) -> Result<FundsAmount> {
    let amount = amount.normalize();

    if amount.is_sign_negative() || amount.is_zero() {
        return Err(anyhow!("{} amount must be positive (>0)", amount));
    };

    Ok(FundsAmount(to_base_units(amount, asset_specs.decimals)?))
}

fn to_base_units(decimal: Decimal, base_10_exp: u32) -> Result<u64> {
    let multiplier = Decimal::from_i128_with_scale(
        10u64
            .checked_pow(base_10_exp)
            .ok_or_else(|| anyhow!("exp overflow decimal: {}, exp: {}", decimal, base_10_exp))?
            as i128,
        0,
    );

    let base_units = (decimal * multiplier).normalize();
    if base_units.scale() != 0 {
        return Err(anyhow!(
            "Amount: {} has more fractional digits than allowed: {}",
            decimal,
            base_10_exp
        ));
    }

    if base_units > Decimal::from_i128_with_scale(u64::MAX as i128, 0) {
        return Err(anyhow!(
            "Base units: {} overflow, decimal: {}, exp: {}",
            base_units,
            decimal,
            base_10_exp
        ));
    }

    base_units
        .to_u64()
        .ok_or_else(|| anyhow!("Couldn't convert decimal: {} to u64", decimal))
}
