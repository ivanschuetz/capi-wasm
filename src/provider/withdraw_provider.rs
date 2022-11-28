use crate::dependencies::FundsAssetSpecs;
use crate::error::FrError;
use crate::js::bridge::log_wrap_new;
use crate::js::common::SignedTxFromJs;
use crate::js::to_sign_js::ToSignJs;
use crate::service::number_formats::validate_funds_amount_input;
use algonaut::core::Address;
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use mbase::models::funds::FundsAmount;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;
use super::withdrawal_history_provider::WithdrawalViewData;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WithdrawProvider {
    async fn txs(&self, pars: WithdrawParJs) -> Result<WithdrawResJs, FrError>;
    async fn submit(&self, pars: SubmitWithdrawParJs) -> Result<SubmitWithdrawResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct WithdrawParJs {
    pub dao_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct WithdrawResJs {
    pub to_sign: ToSignJs,
    pub pt: SubmitWithdrawPassthroughParJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitWithdrawParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitWithdrawPassthroughParJs,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct SubmitWithdrawPassthroughParJs {
    pub inputs: WithdrawInputsPassthroughJs,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct WithdrawInputsPassthroughJs {
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedWithdrawalInputs {
    pub sender: Address,
    pub amount: FundsAmount,
    pub description: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitWithdrawResJs {
    pub saved_withdrawal: WithdrawalViewData,
}

pub fn validate_withdrawal_inputs(
    inputs: &WithdrawInputsPassthroughJs,
    asset_specs: &FundsAssetSpecs,
) -> Result<ValidatedWithdrawalInputs> {
    Ok(ValidatedWithdrawalInputs {
        sender: inputs.sender.parse().map_err(Error::msg)?,
        amount: validate_funds_amount_input(&inputs.withdrawal_amount, asset_specs)?,
        description: inputs.description.clone(),
    })
}

#[wasm_bindgen]
pub async fn withdraw(pars: WithdrawParJs) -> Result<WithdrawResJs, FrError> {
    log_wrap_new("withdraw", pars, async move |pars| {
        providers()?.withdraw.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitWithdraw)]
pub async fn submit_withdraw(pars: SubmitWithdrawParJs) -> Result<SubmitWithdrawResJs, FrError> {
    log_wrap_new("submit_withdraw", pars, async move |pars| {
        providers()?.withdraw.submit(pars).await
    })
    .await
}
