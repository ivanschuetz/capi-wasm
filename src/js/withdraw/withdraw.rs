use super::submit_withdraw::SubmitWithdrawPassthroughParJs;
use crate::{
    dependencies::api,
    js::{
        common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
        withdraw::submit_withdraw::{validate_withdrawal_inputs, WithdrawInputsPassthroughJs},
    },
    service::drain_if_needed::drain_if_needed_txs,
};
use algonaut::core::MicroAlgos;
use anyhow::{Error, Result};
use core::{
    dependencies::algod,
    flows::withdraw::withdraw::{withdraw, WithdrawalInputs},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_withdraw, pars: {:?}", pars);
    to_bridge_res(_bridge_withdraw(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_withdraw(pars: WithdrawParJs) -> Result<WithdrawResJs> {
    log::debug!("_bridge_withdraw, pars: {:?}", pars);

    let algod = algod();
    let api = api();

    let inputs_par = WithdrawInputsPassthroughJs {
        project_uuid: pars.project_uuid.clone(),
        sender: pars.sender.clone(),
        withdrawal_amount: pars.withdrawal_amount.clone(),
        description: pars.description.clone(),
    };
    // just for validation (the result is not used) - inputs are passed through to submit, which validates them again and processes them.
    validate_withdrawal_inputs(&inputs_par)?;

    let project = api.load_project_with_uuid(&pars.project_uuid).await?;

    // TODO we could check balance first (enough to withdraw) but then more requests? depends on which state is more likely, think about this

    let inputs = &WithdrawalInputs {
        project_uuid: pars.project_uuid.parse()?,
        amount: MicroAlgos(pars.withdrawal_amount.parse()?),
        description: pars.description,
    };

    let to_sign_for_withdrawal = withdraw(
        &algod,
        pars.sender.parse().map_err(Error::msg)?,
        inputs,
        &project.central_escrow,
    )
    .await?;

    let mut to_sign = vec![to_sign_for_withdrawal.pay_withdraw_fee_tx];

    let maybe_to_sign_for_drain =
        drain_if_needed_txs(&algod, &project, &pars.sender.parse().map_err(Error::msg)?).await?;
    // we append drain at the end since it's optional, so the indices of the non optional txs are fixed
    let mut maybe_drain_tx_msg_pack = None;
    if let Some(to_sign_for_drain) = maybe_to_sign_for_drain {
        to_sign.push(to_sign_for_drain.pay_fee_tx);
        to_sign.push(to_sign_for_drain.app_call_tx);
        maybe_drain_tx_msg_pack = Some(rmp_serde::to_vec_named(&to_sign_for_drain.drain_tx)?);
    }

    Ok(WithdrawResJs {
        to_sign: to_my_algo_txs1(&to_sign).map_err(Error::msg)?,
        pt: SubmitWithdrawPassthroughParJs {
            maybe_drain_tx_msg_pack,
            withdraw_tx_msg_pack: rmp_serde::to_vec_named(&to_sign_for_withdrawal.withdraw_tx)?,
            inputs: inputs_par.clone(),
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawParJs {
    pub project_uuid: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitWithdrawPassthroughParJs,
}
