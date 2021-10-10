use super::submit_withdraw::SubmitWithdrawPassthroughParJs;
use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1},
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

    let to_sign = withdraw(
        &algod,
        pars.sender.parse().map_err(Error::msg)?,
        MicroAlgos(pars.withdrawal_amount.parse()?),
        project.votes_asset_id,
        &project.central_escrow,
        &project.votein_escrow,
        &project.vote_out_escrow,
    )
    .await?;

    Ok(WithdrawResJs {
        to_sign: to_my_algo_txs1(&vec![to_sign.pay_withdraw_fee_tx, to_sign.pay_vote_fee_tx])
            .map_err(Error::msg)?,
        pt: SubmitWithdrawPassthroughParJs {
            withdraw_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.withdraw_tx)?,
            consume_votes_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.consume_votes_tx)?,
        },
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawParJs {
    pub project_id: String,
    pub sender: String,
    pub withdrawal_amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitWithdrawPassthroughParJs,
}
