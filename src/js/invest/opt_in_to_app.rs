use crate::{
    dependencies::{algod, environment},
    js::common::{to_js_value, to_my_algo_txs},
    service::constants::WITHDRAWAL_SLOT_COUNT,
};
use algonaut::{
    algod::v2::Algod,
    core::Address,
    transaction::{tx_group::TxGroup, Transaction},
};
use anyhow::{anyhow, Result};
use make::flows::shared::app::optin_to_app;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_opt_in_to_apps_if_needed(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_opt_in_to_apps_if_needed, pars: {:?}", pars);

    let algod = algod(&environment());

    let pars = pars.into_serde::<OptInToAppParJs>().map_err(to_js_value)?;

    let res = if is_opted_in(
        &algod,
        pars.investor_address.parse()?,
        pars.app_id.parse().map_err(to_js_value)?,
    )
    .await
    .map_err(to_js_value)?
    {
        OptInToAppResJs { to_sign: None }
        // WARNING: assumption: not opted in to central -> not opted in to all apps
        // normally this should be the case, but user can clear local state of individual apps externally or there can be bugs,
        // TODO: define behavior if app's opted in status varies and implement
        // easiest is probably to return an error ("contact support") -- returning partial opt-ins is finicky
        // and opting the user out requires an additional step on the UI to sign the txs -- this seems the best solution though
    } else {
        let mut parsed_slot_ids = vec![];
        for slot_id in pars.slot_ids {
            parsed_slot_ids.push(slot_id.parse().map_err(to_js_value)?);
        }

        let optins = optin_to_all_apps(
            &algod,
            &pars.investor_address.parse().map_err(to_js_value)?,
            pars.app_id.parse().map_err(to_js_value)?,
            parsed_slot_ids,
        )
        .await
        .map_err(to_js_value)?;

        // sanity check
        if optins.len() != 1 + WITHDRAWAL_SLOT_COUNT as usize {
            return Err(anyhow!(
                "Invalid generated app optins count: {}",
                optins.len()
            ))
            .map_err(to_js_value);
        }

        OptInToAppResJs {
            to_sign: Some(to_my_algo_txs(&optins)?),
        }
    };

    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
}

async fn optin_to_all_apps(
    algod: &Algod,
    investor_address: &Address,
    central_app_id: u64,
    slot_ids: Vec<u64>,
) -> Result<Vec<Transaction>> {
    let params = algod.suggested_transaction_params().await?;
    let mut txs = vec![];
    txs.push(optin_to_app(&params, central_app_id, *investor_address).await?);
    for slot_id in slot_ids {
        txs.push(optin_to_app(&params, slot_id, *investor_address).await?);
    }
    TxGroup::assign_group_id(txs.iter_mut().collect())?;

    Ok(txs)
}

async fn is_opted_in(algod: &Algod, address: Address, app_id: u64) -> Result<bool> {
    let investor_account_infos = algod.account_information(&address).await?;

    // TODO confirm that opted in -> existing local state
    Ok(investor_account_infos
        .apps_local_state
        .iter()
        .any(|a| a.id == app_id))
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct OptInToAppParJs {
    pub app_id: String,
    pub slot_ids: Vec<String>,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptInToAppResJs {
    pub to_sign: Option<Vec<Value>>,
}
