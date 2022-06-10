use crate::{
    error::FrError,
    inputs_validation::ValidationError,
    js::{
        common::{to_js_value, SignedTxFromJs},
        inputs_validation_js::{to_validation_error_js, ValidationErrorJs},
        to_sign_js::ToSignJs,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BuySharesProvider {
    async fn txs(
        &self,
        pars: InvestParJs,
    ) -> Result<InvestResJs, ValidationBuySharesInputsOrAnyhowError>;
    async fn submit(&self, pars: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs, FrError>;
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct InvestParJs {
    pub dao_id: String,
    pub share_count: String,
    pub investor_address: String,
    // not set if the user was already opted in (checked in previous step)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvestResJs {
    pub to_sign: ToSignJs,
    pub pt: SubmitBuySharesPassthroughParJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitBuySharesParJs {
    pub investor_address: String,
    // in case that the transaction fails, to calculate how much we offer the user to buy on on-ramp
    pub buy_amount: String,
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitBuySharesPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBuySharesPassthroughParJs {
    pub dao_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitBuySharesResJs {
    pub message: String,
}

// validation

#[derive(Debug)]
pub enum ValidationBuySharesInputsOrAnyhowError {
    Validation(ValidateBuySharesInputsError),
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for ValidationBuySharesInputsOrAnyhowError {
    fn from(e: anyhow::Error) -> Self {
        ValidationBuySharesInputsOrAnyhowError::Anyhow(e)
    }
}

impl From<ValidateBuySharesInputsError> for ValidationBuySharesInputsOrAnyhowError {
    fn from(e: ValidateBuySharesInputsError) -> Self {
        ValidationBuySharesInputsOrAnyhowError::Validation(e)
    }
}

impl From<ValidationBuySharesInputsOrAnyhowError> for JsValue {
    fn from(e: ValidationBuySharesInputsOrAnyhowError) -> Self {
        match e {
            ValidationBuySharesInputsOrAnyhowError::Validation(e) => e.into(),
            ValidationBuySharesInputsOrAnyhowError::Anyhow(e) => to_js_value(e),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum ValidateBuySharesInputsError {
    Validation(ValidationError),
    NonValidation(String),
}

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize)]
pub struct SharesInputErrorsJs {
    // just some string to identify the struct in js
    pub type_identifier: String,
    pub amount: ValidationErrorJs,
}

impl From<ValidateBuySharesInputsError> for JsValue {
    fn from(error: ValidateBuySharesInputsError) -> JsValue {
        match error {
            ValidateBuySharesInputsError::Validation(e) => {
                let error_js = SharesInputErrorsJs {
                    type_identifier: "input_errors".to_owned(),
                    amount: to_validation_error_js(e),
                };

                match JsValue::from_serde(&error_js) {
                    Ok(js) => js,
                    Err(e) => to_js_value(e),
                }
            }
            _ => to_js_value(format!("Error processing inputs: {error:?}")),
        }
    }
}
