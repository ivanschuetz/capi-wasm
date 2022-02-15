use core::dependencies::algod;
use core::state::account_state::funds_holdings_from_account;

use crate::dependencies::funds_asset_specs;
use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::service::str_to_algos::{base_units_to_display_units_str, microalgos_to_algos_str};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_balance(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_balance, pars: {:?}", pars);
    to_bridge_res(_bridge_balance(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_balance(pars: BalanceParJs) -> Result<BalanceResJs> {
    let algod = algod();
    let funds_asset_specs = funds_asset_specs();

    let account = algod
        .account_information(&pars.address.parse().map_err(Error::msg)?)
        .await?;

    let balance = account.amount;

    let funds_asset_holdings = funds_holdings_from_account(&account, funds_asset_specs.id)?;

    Ok(BalanceResJs {
        balance_algos: microalgos_to_algos_str(balance),
        balance_funds_asset: base_units_to_display_units_str(
            funds_asset_holdings,
            &funds_asset_specs,
        ),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceParJs {
    pub address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceResJs {
    pub balance_algos: String,
    pub balance_funds_asset: String,
}
