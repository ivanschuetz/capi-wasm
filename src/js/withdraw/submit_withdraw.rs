use crate::{
    dependencies::{algod, api, environment},
    js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, SignedTxFromJs},
    service::drain_if_needed::submit_drain,
};
use anyhow::{anyhow, Result};
use core::flows::withdraw::logic::{submit_withdraw, WithdrawSigned};
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

    // 2 txs if only withdrawal, 4 if withdrawal + drain
    if pars.txs.len() != 2 && pars.txs.len() != 4 {
        return Err(anyhow!(
            "Unexpected withdraw txs length: {}",
            pars.txs.len()
        ));
    }
    // sanity check
    if pars.txs.len() == 2 {
        if pars.pt.maybe_drain_tx_msg_pack.is_some() {
            return Err(anyhow!(
                "Invalid state: 2 txs with a passthrough draining tx",
            ));
        }
    }

    if pars.txs.len() == 4 {
        submit_drain(
            &algod,
            &pars.pt.maybe_drain_tx_msg_pack
                .ok_or(anyhow!("Invalid state: if there are signed (in js) drain txs there should be also a passthrough signed drain tx"))?,
            &pars.txs[2],
            &pars.txs[3],
        )
        .await?;
    }

    let pay_withdraw_fee_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
    let check_enough_votes_tx = signed_js_tx_to_signed_tx1(&pars.txs[1])?;

    let withdraw_tx_id = submit_withdraw(
        &algod,
        &WithdrawSigned {
            withdraw_tx: rmp_serde::from_slice(&pars.pt.withdraw_tx_msg_pack)?,
            pay_withdraw_fee_tx,
            check_enough_votes_tx,
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
    // set if a drain tx is necessary
    pub maybe_drain_tx_msg_pack: Option<Vec<u8>>,
    pub withdraw_tx_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitWithdrawResJs {
    pub message: String,
}
