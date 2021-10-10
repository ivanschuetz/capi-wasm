use crate::{dependencies::{algod, api, environment}, js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs}};
use anyhow::{anyhow, Result};
use make::flows::withdraw::logic::{submit_withdraw, WithdrawSigned};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_withdrawal_request(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_withdraw, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_withdrawal_request(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_submit_withdrawal_request(
    pars: SubmitWithdrawParJs,
) -> Result<SubmitWithdrawResJs> {
    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    if pars.txs.len() != 2 {
        return Err(anyhow!(
            "Unexpected signed withdraw txs length: {}",
            pars.txs.len()
        ));
    }

    let pay_withdraw_fee_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
    let pay_vote_fee_tx = signed_js_tx_to_signed_tx1(&pars.txs[1])?;

    let withdraw_tx_id = submit_withdraw(
        &algod,
        &WithdrawSigned {
            withdraw_tx: rmp_serde::from_slice(&pars.pt.withdraw_tx_msg_pack)?,
            pay_withdraw_fee_tx,
            consume_votes_tx: rmp_serde::from_slice(&pars.pt.consume_votes_tx_msg_pack)?,
            pay_vote_fee_tx,
        },
    )
    .await?;

    log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

    api.complete_withdrawal_request(&pars.request_id).await?;

    Ok(SubmitWithdrawResJs {
        message: "Success, withdrawal success!".to_owned(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitWithdrawParJs {
    pub request_id: String,
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitWithdrawPassthroughParJs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitWithdrawPassthroughParJs {
    pub withdraw_tx_msg_pack: Vec<u8>,
    pub consume_votes_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitWithdrawResJs {
    pub message: String,
}
