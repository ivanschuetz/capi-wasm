use crate::{
    dependencies::capi_deps,
    js::common::{parse_bridge_pars, to_bridge_res},
    teal::programs,
};
use anyhow::{anyhow, Error, Result};
use core::{
    dependencies::algod,
    flows::create_dao::{share_amount::ShareAmount, storage::load_dao::load_dao},
    state::{account_state::asset_holdings, app_state::ApplicationLocalStateError, dao_app_state::dao_investor_state},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_my_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_my_shares, pars: {:?}", pars);
    to_bridge_res(_bridge_my_shares(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_my_shares(pars: MySharesParJs) -> Result<MySharesResJs> {
    let algod = algod();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let dao_id = pars.dao_id.parse()?;

    let dao = load_dao(&algod, dao_id, &programs.escrows, &capi_deps).await?;

    log::debug!("Dao: {dao:?}");

    let my_address = &pars.my_address.parse().map_err(Error::msg)?;

    let locked_shares = match dao_investor_state(&algod, my_address, dao.app_id).await {
        Ok(state) => state.shares,
        Err(ApplicationLocalStateError::NotOptedIn) => ShareAmount::new(0), // not invested -> 0 shares
        Err(e) => return Err(Error::msg(e)),
    };

    let free_shares = match asset_holdings(&algod, my_address, dao.shares_asset_id).await {
        Ok(shares) => ShareAmount(shares),
        Err(e) => return Err(Error::msg(e)),
    };

    let total_shares = ShareAmount::new(
        locked_shares
            .val()
            .checked_add(free_shares.val())
            .ok_or(anyhow!("Invalid state: locked shares: {locked_shares} + fee_shares: {free_shares} caused an overflow. This is expected to be <= asset supply, which is an u64"))?,
    );

    Ok(MySharesResJs {
        total: total_shares.0.to_string(),
        free: free_shares.to_string(),
        locked: locked_shares.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct MySharesParJs {
    pub dao_id: String,
    pub my_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MySharesResJs {
    pub total: String,
    pub free: String,
    pub locked: String,
}
