use std::convert::TryInto;
use std::str::FromStr;

use crate::calculate_profit_percentage;
use crate::inputs_validation::ValidationError;
use crate::js::common::to_js_value;
use crate::js::inputs_validation_js::{to_validation_error_js, ValidationErrorJs};
use crate::provider::buy_shares::ValidateBuySharesInputsError;
use crate::provider::calculate_total_price::{
    CalculateTotalPriceParJs, CalculateTotalPriceProvider, CalculateTotalPriceResJs,
};
use crate::provider::investment_provider::CalcPriceAndPercSpecs;
use crate::service::number_formats::{
    validate_funds_amount_input, validate_share_count, base_units_to_display_units_readable,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use mbase::models::funds::FundsAmount;
use mbase::models::share_amount::ShareAmount;
use mbase::util::decimal_util::DecimalExt;
use rust_decimal::Decimal;
use serde::Serialize;
use wasm_bindgen::JsValue;

pub struct CalculateTotalPriceDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CalculateTotalPriceProvider for CalculateTotalPriceDef {
    async fn get(
        &self,
        pars: CalculateTotalPriceParJs,
    ) -> Result<CalculateTotalPriceResJs, ValidationCalcTotalPriceOrAnyhowError> {
        let specs: CalcPriceAndPercSpecs =
            rmp_serde::from_slice(&pars.share_specs_msg_pack).map_err(Error::msg)?;

        let validated_price = validate_funds_amount_input(&pars.share_price, &specs.funds_specs)?;
        let validated_share_amount = validate_share_count(&pars.shares_amount)?;

        let available_shares = ShareAmount::new(pars.available_shares.parse().map_err(Error::msg)?);
        let share_supply = ShareAmount::new(pars.share_supply.parse().map_err(Error::msg)?);
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

        let profit_percentage =
            calculate_profit_percentage(validated_share_amount, share_supply, investors_share)?;

        let total_price_display = base_units_to_display_units_readable(total_price, &specs.funds_specs)?;

        Ok(CalculateTotalPriceResJs {
            total_price: total_price_display,
            profit_percentage: profit_percentage.format_percentage(),
        })
    }
}

// validation

// TODO rename and put somewhere else ValidationSharesInputsOrAnyhowError ?
#[derive(Debug)]
pub enum ValidationCalcTotalPriceOrAnyhowError {
    Validation(ValidateCalcTotalPriceInputsError),
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for ValidationCalcTotalPriceOrAnyhowError {
    fn from(e: anyhow::Error) -> Self {
        ValidationCalcTotalPriceOrAnyhowError::Anyhow(e)
    }
}

impl From<ValidateCalcTotalPriceInputsError> for ValidationCalcTotalPriceOrAnyhowError {
    fn from(e: ValidateCalcTotalPriceInputsError) -> Self {
        ValidationCalcTotalPriceOrAnyhowError::Validation(e)
    }
}

impl From<ValidateBuySharesInputsError> for ValidationCalcTotalPriceOrAnyhowError {
    fn from(e: ValidateBuySharesInputsError) -> Self {
        match e {
            ValidateBuySharesInputsError::Validation(e) => e.into(),
            ValidateBuySharesInputsError::NonValidation(e) => {
                ValidationCalcTotalPriceOrAnyhowError::Anyhow(Error::msg(e))
            }
        }
    }
}

impl From<ValidationCalcTotalPriceOrAnyhowError> for JsValue {
    fn from(e: ValidationCalcTotalPriceOrAnyhowError) -> Self {
        match e {
            ValidationCalcTotalPriceOrAnyhowError::Validation(e) => e.into(),
            ValidationCalcTotalPriceOrAnyhowError::Anyhow(e) => to_js_value(e),
        }
    }
}

impl From<ValidationError> for ValidationCalcTotalPriceOrAnyhowError {
    fn from(e: ValidationError) -> Self {
        ValidationCalcTotalPriceOrAnyhowError::Validation(
            ValidateCalcTotalPriceInputsError::Validation(e),
        )
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum ValidateCalcTotalPriceInputsError {
    Validation(ValidationError),
    NonValidation(String),
}

// /// Errors to be shown next to the respective input fields
// #[derive(Debug, Clone, Serialize, Default)]
// pub struct CalcTotalPriceInputErrors {
//     pub name: Option<ValidationError>,
//     pub description: Option<ValidationError>,
// }

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize, Default)]
pub struct CalcTotalPriceErrorsJs {
    // just some string to identify the struct in js
    pub type_identifier: String,
    pub amount: Option<ValidationErrorJs>,
}

impl From<ValidateCalcTotalPriceInputsError> for JsValue {
    fn from(error: ValidateCalcTotalPriceInputsError) -> JsValue {
        match error {
            ValidateCalcTotalPriceInputsError::Validation(e) => {
                let error_js = to_validation_error_js(e);
                match JsValue::from_serde(&error_js) {
                    Ok(js) => js,
                    Err(e) => to_js_value(e),
                }
            }
            _ => to_js_value(format!("Error processing inputs: {error:?}")),
        }
    }
}
