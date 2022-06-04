use crate::{js::common::to_js_value, provider::create_dao_provider::ValidateDaoInputsError};
use serde::Serialize;
use wasm_bindgen::JsValue;

/// "Fr": frontend
/// All errors that can be returned to JS
#[derive(Debug, Clone)]
pub enum FrError {
    NotEnoughAlgos,
    Msg(String), // this is temporary / last resort: we expect to map all the errors to localized error messages in js
}

// temporary: TODO: generalize validation and add a Validation case to FrError, return FrError instead of ValidateDaoInputsError
impl From<ValidateDaoInputsError> for FrError {
    fn from(e: ValidateDaoInputsError) -> Self {
        FrError::Msg(format!("{:?}", e))
    }
}

impl From<FrError> for JsValue {
    fn from(e: FrError) -> Self {
        let error_with_id = match e {
            FrError::NotEnoughAlgos => FrErrorWithId {
                id: "not_enough_algos".to_owned(),
                details: None,
            },
            // FrError::Msg(msg) => FrErrorWithId { id: "msg".to_owned(), details: Some(serde_json::to_value(&msg)) },
            FrError::Msg(msg) => FrErrorWithId {
                id: "msg".to_owned(),
                details: Some(msg),
            },
        };

        match JsValue::from_serde(&error_with_id) {
            Ok(js) => js,
            Err(e) => to_js_value(e),
        }
    }
}

impl From<anyhow::Error> for FrError {
    fn from(e: anyhow::Error) -> Self {
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
