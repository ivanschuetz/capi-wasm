use crate::js::common::{signed_js_tx_to_signed_tx, to_js_value, SignedTxFromJs};
use core::{
    dependencies::algod,
    flows::invest::{invest::submit_invest, model::InvestSigned},
    network_util::wait_for_pending_transaction,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_buy_shares, pars: {:?}", pars);

    let algod = algod();

    let pars = pars
        .into_serde::<SubmitBuySharesParJs>()
        .map_err(to_js_value)?;

    if pars.txs.len() != 3 {
        return Err(JsValue::from_str(&format!(
            "Unexpected signed invest txs length: {}",
            pars.txs.len()
        )));
    }

    let central_app_setup_tx = signed_js_tx_to_signed_tx(&pars.txs[0])?;
    let payment_tx = signed_js_tx_to_signed_tx(&pars.txs[1])?;
    let shares_asset_optin_tx = signed_js_tx_to_signed_tx(&pars.txs[2])?;

    let dao = rmp_serde::from_slice(&pars.pt.dao_msg_pack).map_err(to_js_value)?;

    let submit_res = submit_invest(
        &algod,
        &InvestSigned {
            dao,
            central_app_setup_tx,
            shares_asset_optin_tx,
            payment_tx,
        },
    )
    .await
    .map_err(to_js_value)?;

    let _ = wait_for_pending_transaction(&algod, &submit_res.tx_id)
        .await
        .map_err(to_js_value)?;

    log::debug!("Submit invest res: {:?}", submit_res);

    let res = SubmitBuySharesResJs {
        message: "Success, you bought some shares!".to_owned(),
    };
    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitBuySharesParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitBuySharesPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitBuySharesPassthroughParJs {
    pub dao_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitBuySharesResJs {
    pub message: String,
}
