use crate::{
    dependencies::{algod, api, environment},
    js::common::{signed_js_tx_to_signed_tx1, to_js_value, to_my_algo_txs, SignedTxFromJs},
};
use algonaut::{algod::v2::Algod, core::ToMsgPack};
use anyhow::{Error, Result};
use make::{flows::invest::logic::invest_txs, network_util::wait_for_pending_transaction};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

use super::submit_buy_shares::SubmitBuySharesPassthroughParJs;

#[wasm_bindgen]
pub async fn bridge_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_buy_shares, pars: {:?}", pars);

    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    let pars = pars.into_serde::<InvestParJs>().map_err(to_js_value)?;

    if let Some(app_opt_in_tx) = pars.app_opt_in_tx {
        submit_app_opt_in(&algod, app_opt_in_tx)
            .await
            .map_err(to_js_value)?;
    }

    log::debug!("Loading the project...");

    let project = api
        .load_project(&pars.project_id)
        .await
        .map_err(to_js_value)?;

    let to_sign = invest_txs(
        &algod,
        &project,
        &pars.investor_address.parse()?,
        &project.staking_escrow,
        project.central_app_id,
        project.shares_asset_id,
        pars.share_count.parse().map_err(to_js_value)?,
        project.specs.asset_price,
    )
    .await
    .map_err(to_js_value)?;

    let res: InvestResJs = InvestResJs {
        to_sign: to_my_algo_txs(&vec![
            to_sign.central_app_opt_in_tx,
            to_sign.payment_tx,
            to_sign.shares_asset_optin_tx,
            to_sign.pay_escrow_fee_tx,
        ])?,
        pt: SubmitBuySharesPassthroughParJs {
            project: project.into(),
            shares_xfer_tx_msg_pack: to_sign.shares_xfer_tx.to_msg_pack().map_err(to_js_value)?,
            votes_xfer_tx_msg_pack: to_sign.votes_xfer_tx.to_msg_pack().map_err(to_js_value)?,
        },
    };
    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
}

async fn submit_app_opt_in(algod: &Algod, tx: SignedTxFromJs) -> Result<()> {
    log::debug!("Submitting app opt-in...");
    let app_opt_in_tx_res = algod
        .broadcast_signed_transaction(&signed_js_tx_to_signed_tx1(&tx).map_err(Error::msg)?)
        .await?;
    let _ = wait_for_pending_transaction(&algod, &app_opt_in_tx_res.tx_id).await?;
    Ok(())
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct InvestParJs {
    pub project_id: String,
    pub share_count: String,
    pub investor_address: String,
    // not set if the user was already opted in (checked in previous step)
    pub app_opt_in_tx: Option<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitBuySharesPassthroughParJs,
}
