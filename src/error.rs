use std::{collections::HashMap, error::Error, num::ParseIntError, string::FromUtf8Error};

use crate::{
    inputs_validation::ValidationError,
    provider::{
        create_dao_provider::{CreateAssetsInputErrors, ValidateDaoInputsError},
        def::update_data_provider_def::{
            ValidateDataUpdateInputsError, ValidateUpateDataInputErrors,
        },
    },
};
use algonaut::error::ServiceError;
use mbase::state::app_state::ApplicationLocalStateError;
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::JsValue;

/// "Fr": frontend
/// All errors that can be returned to JS
#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
#[serde(rename_all(serialize = "camelCase"))]
pub enum FrError {
    NotEnoughAlgos,
    NotEnoughFundsAsset { to_buy: String },
    Validation(ValidationError),
    CreateDaoValidations(CreateAssetsInputErrors),
    UpdateDaoDataValidations(ValidateUpateDataInputErrors),
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

impl From<ValidateDataUpdateInputsError> for FrError {
    fn from(e: ValidateDataUpdateInputsError) -> Self {
        match e {
            ValidateDataUpdateInputsError::AllFieldsValidation(errors) => {
                FrError::UpdateDaoDataValidations(errors)
            }
            ValidateDataUpdateInputsError::NonValidation(msg) => FrError::Msg(msg),
        }
    }
}

// export type Foo = {value: string, kind: "foo"} | {kind: "bar", value: { a: { b: string }} }

pub enum Foo {}

impl From<ValidationError> for FrError {
    fn from(e: ValidationError) -> Self {
        FrError::Validation(e)
    }
}

impl From<FrError> for JsValue {
    fn from(error: FrError) -> Self {
        match JsValue::from_serde(&error) {
            Ok(js_value) => js_value,
            // Err(e) => JsValue::from_str(&format!("{}", e)),
            // For now just panic. Shouldn't happen. FrError should be always serializable.
            // Problem with returning serialization error as JsValue is that it breaks the TS FrError type.
            // if we assume that we *always* get an FrError, we just have to handle FrError (i.e. the enum cases) in TS.
            Err(e) => {
                panic!("Unexpected error serializing FrError: {}", e)
            }
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
