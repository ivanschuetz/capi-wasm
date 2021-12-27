use super::withdrawal_history::WithdrawalViewData;
use crate::{
    js::{
        common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs},
        withdraw::withdrawal_view_data,
    },
    service::{drain_if_needed::submit_drain, str_to_algos::validate_algos_input},
};
use algonaut::core::{Address, MicroAlgos};
use anyhow::{anyhow, Error, Result};
use core::{
    dependencies::algod,
    flows::withdraw::withdraw::{submit_withdraw, WithdrawSigned},
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

    let withdrawal_inputs = validate_withdrawal_inputs(&pars.pt.inputs)?;

    // 1 tx if only withdrawal, 3 if withdrawal + drain
    if pars.txs.len() != 1 && pars.txs.len() != 3 {
        return Err(anyhow!(
            "Unexpected withdraw txs length: {}",
            pars.txs.len()
        ));
    }
    // sanity check
    if pars.txs.len() == 1 && pars.pt.maybe_drain_tx_msg_pack.is_some() {
        return Err(anyhow!(
            "Invalid state: 2 txs with a passthrough draining tx",
        ));
    }

    if pars.txs.len() == 3 {
        submit_drain(
            &algod,
            &pars.pt.maybe_drain_tx_msg_pack
                .ok_or_else(|| anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed drain tx"))?,
            &pars.txs[1],
            &pars.txs[2],
        )
        .await?;
    }

    let pay_withdraw_fee_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;

    let withdraw_tx_id = submit_withdraw(
        &algod,
        &WithdrawSigned {
            withdraw_tx: rmp_serde::from_slice(&pars.pt.withdraw_tx_msg_pack)?,
            pay_withdraw_fee_tx,
        },
    )
    .await?;

    log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

    Ok(SubmitWithdrawResJs {
        saved_withdrawal: withdrawal_view_data(
            withdrawal_inputs.amount,
            withdrawal_inputs.description,
            "Just now".to_owned(),
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
    pub withdraw_tx_msg_pack: Vec<u8>,

    pub inputs: WithdrawInputsPassthroughJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawInputsPassthroughJs {
    pub project_uuid: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedWithdrawalInputs {
    pub project_id: String,
    pub sender: Address,
    pub amount: MicroAlgos,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitWithdrawResJs {
    pub saved_withdrawal: WithdrawalViewData,
}

pub fn validate_withdrawal_inputs(
    inputs: &WithdrawInputsPassthroughJs,
) -> Result<ValidatedWithdrawalInputs> {
    Ok(ValidatedWithdrawalInputs {
        project_id: inputs.project_uuid.parse()?,
        sender: inputs.sender.parse().map_err(Error::msg)?,
        amount: validate_algos_input(&inputs.withdrawal_amount)?,
        description: inputs.description.clone(),
    })
}
