use crate::{dependencies::{algod, environment}, js::common::{to_js_value, to_my_algo_tx}};
use algonaut::{algod::v2::Algod, core::Address};
use anyhow::Result;
use make::flows::shared::app::optin_to_app;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_opt_in_to_app_if_needed(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_opt_in_to_app, pars: {:?}", pars);

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
    } else {
        let params = algod
            .suggested_transaction_params()
            .await
            .map_err(to_js_value)?;
        let app_optin_tx = optin_to_app(
            &params,
            pars.app_id.parse().map_err(to_js_value)?,
            pars.investor_address.parse().map_err(to_js_value)?,
        )
        .await
        .map_err(to_js_value)?;

        OptInToAppResJs {
            to_sign: Some(to_my_algo_tx(&app_optin_tx)?),
        }
    };

    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
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
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptInToAppResJs {
    pub to_sign: Option<Value>,
}
