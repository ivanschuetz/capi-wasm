use crate::dependencies::FundsAssetSpecs;
use crate::js::common::SignedTxFromJs;
use crate::service::number_formats::validate_funds_amount_input;
use algonaut::core::Address;
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use mbase::models::funds::FundsAmount;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

use super::withdrawal_history_provider::WithdrawalViewData;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WithdrawProvider {
    async fn txs(&self, pars: WithdrawParJs) -> Result<WithdrawResJs>;
    async fn submit(&self, pars: SubmitWithdrawParJs) -> Result<SubmitWithdrawResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawParJs {
    pub dao_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitWithdrawPassthroughParJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitWithdrawParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitWithdrawPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitWithdrawPassthroughParJs {
    // set if a drain tx is necessary
    pub maybe_drain_tx_msg_pack: Option<Vec<u8>>,
    pub maybe_capi_share_tx_msg_pack: Option<Vec<u8>>,

    pub inputs: WithdrawInputsPassthroughJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize)]
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
