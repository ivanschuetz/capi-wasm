use crate::{
    error::FrError,
    inputs_validation::ValidationError,
    js::{
        bridge::log_wrap_new,
        common::{to_js_value, SignedTxFromJs},
        inputs_validation_js::{to_validation_error_js, ValidationErrorJs},
        to_sign_js::ToSignJs,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use tsify::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BuySharesProvider {
    async fn txs(&self, pars: InvestParJs) -> Result<InvestResJs, FrError>;
    async fn submit(&self, pars: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs, FrError>;
}

// TODO rename structs in BuyShares*
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct InvestParJs {
    pub dao_id: String,
    pub share_count: String,
    pub investor_address: String,
    // not set if the user was already opted in (checked in previous step)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
    // passed as parameter (reuse UI value), to prevent 2 additional requests to calculate them
    pub available_shares: String,

    pub signed_prospectus: SignedProspectusJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignedProspectusJs {
    pub url: String,
    pub hash: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct InvestResJs {
    pub to_sign: ToSignJs,
    pub pt: SubmitBuySharesPassthroughParJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitBuySharesParJs {
    pub investor_address: String,
    // in case that the transaction fails, to calculate how much we offer the user to buy on on-ramp
    pub buy_total_cost: String,
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitBuySharesPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBuySharesPassthroughParJs {
    pub dao_msg_pack: Vec<u8>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitBuySharesResJs {
    pub message: String,
}

// validation

#[derive(Debug)]
pub enum ValidationBuySharesInputsOrAnyhowError {
    Validation(ValidateSharesInputError),
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for ValidationBuySharesInputsOrAnyhowError {
    fn from(e: anyhow::Error) -> Self {
        ValidationBuySharesInputsOrAnyhowError::Anyhow(e)
    }
}

impl From<ValidateSharesInputError> for ValidationBuySharesInputsOrAnyhowError {
    fn from(e: ValidateSharesInputError) -> Self {
        ValidationBuySharesInputsOrAnyhowError::Validation(e)
    }
}

impl From<ValidationError> for ValidationBuySharesInputsOrAnyhowError {
    fn from(e: ValidationError) -> Self {
        ValidateSharesInputError::Validation(e).into()
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
pub enum ValidateSharesInputError {
    Validation(ValidationError),
    // NonValidation(String),
}

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize)]
pub struct SharesInputErrorsJs {
    // just some string to identify the struct in js
    pub type_identifier: String,
    pub amount: ValidationErrorJs,
}

impl From<ValidateSharesInputError> for JsValue {
    fn from(error: ValidateSharesInputError) -> JsValue {
        match error {
            ValidateSharesInputError::Validation(e) => {
                let error_js = SharesInputErrorsJs {
                    type_identifier: "input_errors".to_owned(),
                    amount: to_validation_error_js(e),
                };

                match JsValue::from_serde(&error_js) {
                // match to_value(&error_js) {
                    Ok(js) => js,
                    Err(e) => to_js_value(e),
                }
            }
        }
    }
}

impl From<ValidateSharesInputError> for FrError {
    fn from(error: ValidateSharesInputError) -> FrError {
        match error {
            ValidateSharesInputError::Validation(e) => FrError::Validation(e),
        }
    }
}

#[wasm_bindgen(js_name=buyShares)]
pub async fn buy_shares(pars: InvestParJs) -> Result<InvestResJs, FrError> {
    log_wrap_new("buy_shares", pars, async move |pars| {
        providers()?.buy_shares.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitBuyShares)]
pub async fn submit_buy_shares(
    pars: SubmitBuySharesParJs,
) -> Result<SubmitBuySharesResJs, FrError> {
    log_wrap_new("submit_buy_shares", pars, async move |pars| {
        providers()?.buy_shares.submit(pars).await
    })
    .await
}
