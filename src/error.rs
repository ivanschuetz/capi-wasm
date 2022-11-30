use std::{collections::HashMap, error::Error, num::ParseIntError, string::FromUtf8Error};

use crate::{
    inputs_validation::ValidationError,
    js::{
        common::to_js_res,
        inputs_validation_js::{to_validation_error_js, ValidationErrorJs},
    },
    provider::{
        create_assets_provider::CreateAssetsInputErrorsJs,
        create_dao_provider::{CreateAssetsInputErrors, ValidateDaoInputsError},
    },
};
use algonaut::error::ServiceError;
use mbase::{models::asset_amount::AssetAmount, state::app_state::ApplicationLocalStateError};
use serde::Serialize;
use wasm_bindgen::JsValue;

/// "Fr": frontend
/// All errors that can be returned to JS
#[derive(Debug, Clone)]
pub enum FrError {
    NotEnoughAlgos,
    NotEnoughFundsAsset { to_buy: AssetAmount },
    Validation(ValidationError),
    CreateDaoValidations(CreateAssetsInputErrors),
    Validations(HashMap<String, ValidationError>),
    Internal(String), // Things we can't explain to users. Text is for developers (can be forwarded with error reporting).
    Msg(String), // this is temporary / last resort: we expect to map all the errors to localized error messages in js
}

// temporary: TODO: generalize validation and add a Validation case to FrError, return FrError instead of ValidateDaoInputsError
impl From<ValidateDaoInputsError> for FrError {
    fn from(e: ValidateDaoInputsError) -> Self {
        match e {
            ValidateDaoInputsError::AllFieldsValidation(errors) => {
                FrError::CreateDaoValidations(errors)
            }
            ValidateDaoInputsError::NonValidation(msg) => FrError::Msg(msg),
        }
    }
}

impl From<ValidationError> for FrError {
    fn from(e: ValidationError) -> Self {
        FrError::Validation(e)
    }
}

impl From<FrError> for JsValue {
    fn from(error: FrError) -> Self {
        match try_from(error) {
            Ok(js_value) => js_value,
            Err(js_value) => js_value,
        }
    }
}

fn try_from(e: FrError) -> Result<JsValue, JsValue> {
    match e {
        FrError::NotEnoughAlgos => to_js_res(FrErrorWithId::<String> {
            id: "not_enough_algos".to_owned(),
            details: None,
        }),
        FrError::NotEnoughFundsAsset { to_buy } => to_js_res(FrErrorWithId {
            id: "not_enough_funds_asset".to_owned(),
            details: Some(to_buy.0.to_string()),
        }),
        // TODO is this not being handled in JS?
        FrError::Msg(msg) => to_js_res(FrErrorWithId {
            id: "msg".to_owned(),
            details: Some(msg),
        }),
        FrError::Internal(msg) => to_js_res(FrErrorWithId {
            id: "internal".to_owned(),
            details: Some(msg),
        }),
        FrError::Validation(validation) => {
            let error_js = to_validation_error_js(validation);
            to_js_res(FrErrorWithId {
                id: "validation".to_owned(),
                details: Some(error_js),
            })
        }
        FrError::Validations(validations) => {
            let map_js: HashMap<String, ValidationErrorJs> = validations
                .into_iter()
                .map(|(k, v)| (k, to_validation_error_js(v)))
                .collect();

            to_js_res(FrErrorWithId {
                id: "validations".to_owned(),
                details: Some(map_js),
            })
        }
        FrError::CreateDaoValidations(e) => {
            let errors_js = CreateAssetsInputErrorsJs {
                type_identifier: "input_errors".to_owned(),
                name: e.name.map(to_validation_error_js),
                description: e.description.map(to_validation_error_js),
                creator: e.creator.map(to_validation_error_js),
                share_supply: e.share_supply.map(to_validation_error_js),
                share_price: e.share_price.map(to_validation_error_js),
                investors_share: e.investors_share.map(to_validation_error_js),
                social_media_url: e.social_media_url.map(to_validation_error_js),
                min_raise_target: e.min_raise_target.map(to_validation_error_js),
                min_raise_target_end_date: e.min_raise_target_end_date.map(to_validation_error_js),
                image_url: e.image_url.map(to_validation_error_js),
                prospectus_url: e.prospectus_url.map(to_validation_error_js),
                prospectus_bytes: e.prospectus_bytes.map(to_validation_error_js),
                min_invest_shares: e.min_invest_amount.map(to_validation_error_js),
                max_invest_shares: e.max_invest_amount.map(to_validation_error_js),
                shares_for_investors: e.shares_for_investors.map(to_validation_error_js),
            };

            to_js_res(FrErrorWithId {
                id: "validations".to_owned(),
                details: Some(&errors_js),
            })
        }
    }
}

impl From<anyhow::Error> for FrError {
    fn from(e: anyhow::Error) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<ServiceError> for FrError {
    fn from(e: ServiceError) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<ParseIntError> for FrError {
    fn from(e: ParseIntError) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<rust_decimal::Error> for FrError {
    fn from(e: rust_decimal::Error) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<FromUtf8Error> for FrError {
    fn from(e: FromUtf8Error) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<serde_json::Error> for FrError {
    fn from(e: serde_json::Error) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<Box<dyn Error + 'static>> for FrError {
    fn from(e: Box<dyn Error + 'static>) -> Self {
        FrError::Msg(e.to_string())
    }
}

impl From<ApplicationLocalStateError<'static>> for FrError {
    fn from(e: ApplicationLocalStateError) -> Self {
        FrError::Msg(format!("{e:?}"))
    }
}
