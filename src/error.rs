use std::convert::TryFrom;

use crate::{
    inputs_validation::ValidationError,
    js::{common::to_js_res, inputs_validation_js::to_validation_error_js},
    provider::create_dao_provider::ValidateDaoInputsError,
};
use algonaut::error::ServiceError;
use mbase::models::asset_amount::AssetAmount;
use serde::Serialize;
use wasm_bindgen::JsValue;

/// "Fr": frontend
/// All errors that can be returned to JS
#[derive(Debug, Clone)]
pub enum FrError {
    NotEnoughAlgos,
    NotEnoughFundsAsset { to_buy: AssetAmount },
    Validation(ValidationError),
    Msg(String), // this is temporary / last resort: we expect to map all the errors to localized error messages in js
}

// temporary: TODO: generalize validation and add a Validation case to FrError, return FrError instead of ValidateDaoInputsError
impl From<ValidateDaoInputsError> for FrError {
    fn from(e: ValidateDaoInputsError) -> Self {
        FrError::Msg(format!("{:?}", e))
    }
}

impl TryFrom<FrError> for JsValue {
    type Error = JsValue;

    fn try_from(e: FrError) -> Result<Self, Self::Error> {
        match e {
            FrError::NotEnoughAlgos => to_js_res(FrErrorWithId::<String> {
                id: "not_enough_algos".to_owned(),
                details: None,
            }),
            FrError::NotEnoughFundsAsset { to_buy } => to_js_res(FrErrorWithId {
                id: "not_enough_funds_asset".to_owned(),
                details: Some(to_buy.0.to_string()),
            }),
            FrError::Msg(msg) => to_js_res(FrErrorWithId {
                id: "msg".to_owned(),
                details: Some(msg),
            }),
            FrError::Validation(validation) => {
                let error_js = to_validation_error_js(validation);
                to_js_res(FrErrorWithId {
                    id: "validation".to_owned(),
                    details: Some(error_js),
                })
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

#[derive(Debug, Serialize)]
struct FrErrorWithId<T>
where
    T: Serialize,
{
    id: String,
    details: Option<T>,
}
