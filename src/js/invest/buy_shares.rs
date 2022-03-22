use super::submit_buy_shares::SubmitBuySharesPassthroughParJs;
use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1, SignedTxFromJs},
    service::invest_or_lock::submit_apps_optins_from_js,
    teal::programs,
};
use algonaut::core::ToMsgPack;
use anyhow::{anyhow, Error, Result};
use core::{
    dependencies::algod,
    flows::{
        create_dao::{share_amount::ShareAmount, storage::load_dao::load_dao},
        invest::invest::invest_txs,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_buy_shares, pars: {:?}", pars);
    to_bridge_res(_bridge_buy_shares(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_buy_shares(pars: InvestParJs) -> Result<InvestResJs> {
    let algod = algod();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let validated_share_amount = validate_share_count(&pars.share_count)?;

    if let Some(app_opt_ins) = pars.app_opt_ins {
        submit_apps_optins_from_js(&algod, &app_opt_ins).await?;
    }

    log::debug!("Loading the dao...");

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &programs.escrows, &capi_deps).await?;

    let to_sign = invest_txs(
        &algod,
        &dao,
        &pars.investor_address.parse().map_err(Error::msg)?,
        &dao.locking_escrow,
        dao.app_id,
        dao.shares_asset_id,
        validated_share_amount,
        funds_asset_specs().id,
        dao.specs.share_price,
    )
    .await?;

    let to_sign_txs = vec![
        to_sign.central_app_setup_tx,
        to_sign.payment_tx,
        to_sign.shares_asset_optin_tx,
    ];

    Ok(InvestResJs {
        to_sign: to_my_algo_txs1(&to_sign_txs).map_err(Error::msg)?,
        pt: SubmitBuySharesPassthroughParJs {
            dao_msg_pack: rmp_serde::to_vec_named(&dao)?,
            shares_xfer_tx_msg_pack: to_sign.shares_xfer_tx.to_msg_pack()?,
        },
    })
}

fn validate_share_count(input: &str) -> Result<ShareAmount> {
    // TODO < available shares (asset count in investing escrow).
    // maybe we can allow investor to enter only a valid amount, e.g. with stepper or graphically
    let share_count = input.parse()?;
    if share_count == 0 {
        return Err(anyhow!("Please enter a valid share count"));
    }
    Ok(ShareAmount::new(share_count))
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct InvestParJs {
    pub dao_id: String,
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
