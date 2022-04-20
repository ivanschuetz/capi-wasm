use crate::dependencies::funds_asset_specs;
use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::js::general::js_types_workarounds::VersionedContractAccountJs;
use crate::model::dao_for_users::dao_to_dao_for_users;
use crate::model::dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData};
use anyhow::{anyhow, Error, Result};
use core::dependencies::algod;
use core::flows::create_dao::setup_dao_specs::SetupDaoSpecs;
use core::flows::create_dao::storage::load_dao::DaoAppId;
use core::flows::create_dao::{model::SetupDaoSigned, setup_dao::submit_setup_dao};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_create_dao, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_create_dao(parse_bridge_pars(pars)?).await)
}

/// create daos specs + signed assets txs -> create dao result
/// submits the signed assets, creates rest of dao with generated asset ids
async fn _bridge_submit_create_dao(pars: SubmitCreateDaoParJs) -> Result<DaoForUsersViewData> {
    // log::debug!("in bridge_submit_create_dao, pars: {:?}", pars);

    let algod = algod();
    let funds_asset_specs = funds_asset_specs()?;

    if pars.txs.len() != 4 {
        return Err(anyhow!(
            "Unexpected signed dao txs length: {}",
            pars.txs.len()
        ));
    }

    // TODO (low prio) improve this access, it's easy for the indices to get out of sync
    // and assign the txs to incorrect variables, which may cause subtle bugs
    // maybe refactor writing/reading into a helper struct or function
    // (written in create_dao::txs_to_sign)
    let setup_app_tx = &pars.txs[0];
    let customer_escrow_funding_tx = &pars.txs[1];
    let app_funding_tx = &pars.txs[2];
    let transfer_shares_to_app_tx = &pars.txs[3];

    log::debug!("Submitting the dao..");

    let submit_dao_res = submit_setup_dao(
        &algod,
        SetupDaoSigned {
            app_funding_tx: signed_js_tx_to_signed_tx1(app_funding_tx)?,
            fund_customer_escrow_tx: signed_js_tx_to_signed_tx1(customer_escrow_funding_tx)?,
            customer_escrow_optin_to_funds_asset_tx: rmp_serde::from_slice(
                &pars.pt.customer_escrow_optin_to_funds_asset_tx_msg_pack,
            )
            .map_err(Error::msg)?,
            transfer_shares_to_app_tx: signed_js_tx_to_signed_tx1(transfer_shares_to_app_tx)?,
            setup_app_tx: signed_js_tx_to_signed_tx1(setup_app_tx)?,
            specs: pars.pt.specs,
            creator: pars.pt.creator.parse().map_err(Error::msg)?,
            shares_asset_id: pars.pt.shares_asset_id,
            customer_escrow: pars.pt.customer_escrow.try_into().map_err(Error::msg)?,
            funds_asset_id: funds_asset_specs.id,
            app_id: DaoAppId(pars.pt.app_id),
        },
    )
    .await?;

    log::debug!("Submit dao res: {:?}", submit_dao_res);

    Ok(dao_for_users_to_view_data(
        dao_to_dao_for_users(&submit_dao_res.dao, &submit_dao_res.dao.id())?,
        &funds_asset_specs,
    ))
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitCreateDaoParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitSetupDaoPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSetupDaoPassthroughParJs {
    pub specs: SetupDaoSpecs,
    // not sure how to passthrough, if we use Address, when deserializing, we get:
    // index.js:1 Error("invalid type: sequence, expected a 32 byte array", line: 1, column: 10711)
    // looking at the logs, the passed JsValue looks like an array ([1, 2...])
    pub creator: String,
    // can't use SignedTransactions because of deserialization issue mainly (only?) with Address
    // see note on `creator` above
    // Note: multiple transactions: the tx vector is serialized into a single u8 vector
    pub customer_escrow_optin_to_funds_asset_tx_msg_pack: Vec<u8>,
    pub shares_asset_id: u64,
    pub customer_escrow: VersionedContractAccountJs,
    pub app_id: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitCreateDaoResJs {
    // next step tx: save the dao
    pub to_sign: Value,
}
