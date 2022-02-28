use super::submit_withdraw::SubmitWithdrawPassthroughParJs;
use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    js::{
        common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
        withdraw::submit_withdraw::{validate_withdrawal_inputs, WithdrawInputsPassthroughJs},
    },
    service::drain_if_needed::drain_if_needed_txs,
    teal::programs,
};
use anyhow::{Error, Result};
use core::{
    dependencies::{algod, indexer},
    flows::{
        create_project::storage::load_project::load_project,
        withdraw::withdraw::{withdraw, WithdrawalInputs},
    },
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
    let indexer = indexer();
    let funds_asset_specs = funds_asset_specs();
    let capi_deps = capi_deps()?;

    let project = load_project(
        &algod,
        &indexer,
        &pars.project_id.parse()?,
        &programs().escrows,
    )
    .await?
    .project;

    let inputs_par = WithdrawInputsPassthroughJs {
        sender: pars.sender.clone(),
        withdrawal_amount: pars.withdrawal_amount.clone(),
        description: pars.description.clone(),
    };

    let validated_inputs = validate_withdrawal_inputs(&inputs_par, &funds_asset_specs)?;

    // TODO we could check balance first (enough to withdraw) but then more requests? depends on which state is more likely, think about this

    let inputs = &WithdrawalInputs {
        amount: validated_inputs.amount,
        description: validated_inputs.description,
    };

    let to_sign_for_withdrawal = withdraw(
        &algod,
        pars.sender.parse().map_err(Error::msg)?,
        funds_asset_specs.id,
        inputs,
        &project.central_escrow,
    )
    .await?;

    let mut to_sign = vec![to_sign_for_withdrawal.pay_withdraw_fee_tx];

    let maybe_to_sign_for_drain = drain_if_needed_txs(
        &algod,
        &project,
        &pars.sender.parse().map_err(Error::msg)?,
        funds_asset_specs.id,
        &capi_deps,
    )
    .await?;
    // we append drain at the end since it's optional, so the indices of the non optional txs are fixed
    let mut maybe_drain_tx_msg_pack = None;
    let mut maybe_capi_share_tx_msg_pack = None;
    if let Some(to_sign_for_drain) = maybe_to_sign_for_drain {
        to_sign.push(to_sign_for_drain.app_call_tx);
        to_sign.push(to_sign_for_drain.capi_app_call_tx);
        maybe_drain_tx_msg_pack = Some(rmp_serde::to_vec_named(&to_sign_for_drain.drain_tx)?);
        maybe_capi_share_tx_msg_pack =
            Some(rmp_serde::to_vec_named(&to_sign_for_drain.capi_share_tx)?);
    }

    Ok(WithdrawResJs {
        to_sign: to_my_algo_txs1(&to_sign).map_err(Error::msg)?,
        pt: SubmitWithdrawPassthroughParJs {
            maybe_drain_tx_msg_pack,
            maybe_capi_share_tx_msg_pack,
            withdraw_tx_msg_pack: rmp_serde::to_vec_named(&to_sign_for_withdrawal.withdraw_tx)?,
            inputs: inputs_par.clone(),
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawParJs {
    pub project_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitWithdrawPassthroughParJs,
}
