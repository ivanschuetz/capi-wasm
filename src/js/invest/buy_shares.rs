use crate::{
    dependencies::{algod, api, environment},
    js::common::{to_js_value, to_my_algo_txs, SignedTxFromJs},
    service::invest_or_stake::submit_apps_optins_from_js,
};
use algonaut::core::ToMsgPack;
use anyhow::{anyhow, Result};
use core::flows::invest::logic::invest_txs;
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

    let validated_share_count = validate_share_count(&pars.share_count).map_err(to_js_value)?;

    if let Some(app_opt_ins) = pars.app_opt_ins {
        submit_apps_optins_from_js(&algod, &app_opt_ins)
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
        validated_share_count,
        project.specs.asset_price,
    )
    .await
    .map_err(to_js_value)?;

    let to_sign_txs = vec![
        to_sign.central_app_setup_tx,
        to_sign.payment_tx,
        to_sign.shares_asset_optin_tx,
        to_sign.pay_escrow_fee_tx,
    ];

    let res: InvestResJs = InvestResJs {
        to_sign: to_my_algo_txs(&to_sign_txs)?,
        pt: SubmitBuySharesPassthroughParJs {
            project: project.into(),
            shares_xfer_tx_msg_pack: to_sign.shares_xfer_tx.to_msg_pack().map_err(to_js_value)?,
        },
    };
    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
}

fn validate_share_count(input: &str) -> Result<u64> {
    // TODO < available shares (asset count in investing escrow).
    // maybe we can allow investor to enter only a valid amount, e.g. with stepper or graphically
    let share_count = input.parse()?;
    if share_count == 0 {
        return Err(anyhow!("Please enter a valid share count"));
    }
    Ok(share_count)
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct InvestParJs {
    pub project_id: String,
    pub share_count: String,
    pub investor_address: String,
    // not set if the user was already opted in (checked in previous step)
    pub app_opt_ins: Option<Vec<SignedTxFromJs>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InvestResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitBuySharesPassthroughParJs,
}
