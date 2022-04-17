use super::withdrawal_history::WithdrawalViewData;
use crate::{
    dependencies::{funds_asset_specs, FundsAssetSpecs},
    js::{
        common::{parse_bridge_pars, to_bridge_res, SignedTxFromJs, signed_js_tx_to_signed_tx1},
        withdraw::withdrawal_view_data,
    },
    service::{drain_if_needed::submit_drain, str_to_algos::validate_funds_amount_input},
};
use algonaut::core::Address;
use anyhow::{anyhow, Error, Result};
use core::{
    dependencies::algod,
    flows::withdraw::withdraw::{submit_withdraw, WithdrawSigned},
    funds::FundsAmount,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_withdraw, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_withdraw(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_withdraw(pars: SubmitWithdrawParJs) -> Result<SubmitWithdrawResJs> {
    let algod = algod();
    let funds_asset_specs = funds_asset_specs()?;

    let withdrawal_inputs = validate_withdrawal_inputs(&pars.pt.inputs, &funds_asset_specs)?;

    // 1 tx if only withdrawal, 3 if withdrawal with drain
    if pars.txs.len() != 1 && pars.txs.len() != 3 {
        return Err(anyhow!(
            "Unexpected withdraw txs length: {}",
            pars.txs.len()
        ));
    }
    // sanity check
    if pars.txs.len() == 1 && pars.pt.maybe_drain_tx_msg_pack.is_some() {
        return Err(anyhow!(
            "Invalid state: 0 txs with a passthrough draining tx",
        ));
    }

    if pars.txs.len() == 3 {
        let drain_tx = &pars.pt.maybe_drain_tx_msg_pack
            .ok_or_else(|| anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed drain tx"))?;

        let capi_share_tx = &pars.pt.maybe_capi_share_tx_msg_pack
            .ok_or_else(|| anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed capi share tx"))?;

        submit_drain(
            &algod,
            &drain_tx,
            &pars.txs[1],
            &capi_share_tx,
            &pars.txs[2],
        )
        .await?;
    }

    let withdraw_tx_id = submit_withdraw(
        &algod,
        &WithdrawSigned {
            withdraw_tx: signed_js_tx_to_signed_tx1(&pars.txs[0])?,
        },
    )
    .await?;

    log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

    Ok(SubmitWithdrawResJs {
        saved_withdrawal: withdrawal_view_data(
            withdrawal_inputs.amount,
            &funds_asset_specs,
            withdrawal_inputs.description,
            "Just now".to_owned(),
            withdraw_tx_id,
        ),
    })
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
