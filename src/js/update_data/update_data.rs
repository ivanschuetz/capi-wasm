use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_tx1};
use algonaut::core::Address;
use anyhow::{Error, Result};
use core::dependencies::algod;
use core::flows::create_dao::storage::load_dao::DaoId;
use core::flows::update_data::update_data::{update_data, UpdatableDaoData};
use core::funds::FundsAmount;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_update_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_update_data, pars: {:?}", pars);
    to_bridge_res(_bridge_update_data(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_update_data(pars: UpdateDataParJs) -> Result<UpdateDataResJs> {
    let algod = algod();

    let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;
    let owner = pars.owner.parse().map_err(Error::msg)?;

    // TODO escrow versioning
    // we're currently saving only the addresses, so don't have the programs to lsig
    // so we've to store the version too (although it could be skipped by just trying all available versions against the address, which seems very inefficient)
    // and use this version to retrieve the program
    // the teal has to be updated to store the version, either in the same field as the address or a separate field with all the escrow's versions

    let to_sign = update_data(
        &algod,
        &owner,
        dao_id.0,
        &UpdatableDaoData {
            central_escrow: parse_addr(pars.central_escrow)?,
            customer_escrow: parse_addr(pars.customer_escrow)?,
            investing_escrow: parse_addr(pars.investing_escrow)?,
            locking_escrow: parse_addr(pars.locking_escrow)?,
            project_name: pars.project_name,
            project_desc: pars.project_desc,
            share_price: FundsAmount::new(pars.share_price.parse().map_err(Error::msg)?),
            logo_url: pars.logo_url,
            social_media_url: pars.social_media_url,
            owner,
        },
    )
    .await?;

    Ok(UpdateDataResJs {
        to_sign: to_my_algo_tx1(&to_sign.update).map_err(Error::msg)?,
    })
}

fn parse_addr(s: String) -> Result<Address> {
    s.parse().map_err(Error::msg)
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDataParJs {
    pub dao_id: String,
    pub owner: String,

    pub central_escrow: String,
    pub customer_escrow: String,
    pub investing_escrow: String,
    pub locking_escrow: String,

    pub project_name: String,
    pub project_desc: String,
    pub share_price: String,

    pub logo_url: String,
    pub social_media_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateDataResJs {
    pub to_sign: Value,
}
