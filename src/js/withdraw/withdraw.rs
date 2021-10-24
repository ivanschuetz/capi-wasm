use super::submit_withdraw::SubmitWithdrawPassthroughParJs;
use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
    service::drain_if_needed::drain_if_needed_txs,
};
use algonaut::core::MicroAlgos;
use anyhow::{Error, Result};
use make::flows::withdraw::logic::withdraw;
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

    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let project = api.load_project(&pars.project_id).await?;

    // TODO we could check balance first (enough to withdraw) but then more requests? depends on which state is more likely, think about this

    let to_sign_for_withdrawal = withdraw(
        &algod,
        pars.sender.parse().map_err(Error::msg)?,
        MicroAlgos(pars.withdrawal_amount.parse()?),
        &project.central_escrow,
        pars.slot_id.parse()?,
    )
    .await?;

    let mut to_sign = vec![];
    to_sign.push(to_sign_for_withdrawal.pay_withdraw_fee_tx);
    to_sign.push(to_sign_for_withdrawal.check_enough_votes_tx);

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
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawParJs {
    pub project_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
    pub slot_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitWithdrawPassthroughParJs,
}
