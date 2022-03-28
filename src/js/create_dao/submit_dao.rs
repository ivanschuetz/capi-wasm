use crate::dependencies::funds_asset_specs;
use crate::js::common::SignedTxFromJs;
use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, signed_js_txs_to_signed_tx1, to_bridge_res,
};
use crate::js::general::js_types_workarounds::VersionedContractAccountJs;
use crate::model::dao_for_users::dao_to_dao_for_users;
use crate::model::dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData};
use anyhow::{anyhow, Error, Result};
use core::dependencies::algod;
use core::flows::create_dao::create_dao_specs::CreateDaoSpecs;
use core::flows::create_dao::storage::load_dao::DaoAppId;
use core::flows::create_dao::{create_dao::submit_create_dao, model::CreateDaoSigned};
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

    if pars.txs.len() != 6 {
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
    let escrow_funding_txs = &pars.txs[1..5];
    let xfer_shares_to_invest_escrow = &pars.txs[5];

    log::debug!("Submitting the dao..");

    let submit_dao_res = submit_create_dao(
        &algod,
        CreateDaoSigned {
            escrow_funding_txs: signed_js_txs_to_signed_tx1(escrow_funding_txs)?,
            optin_txs: rmp_serde::from_slice(&pars.pt.escrow_optin_signed_txs_msg_pack)
                .map_err(Error::msg)?,
            setup_app_tx: signed_js_tx_to_signed_tx1(setup_app_tx)?,
            xfer_shares_to_invest_escrow: signed_js_tx_to_signed_tx1(xfer_shares_to_invest_escrow)?,
            specs: pars.pt.specs,
            creator: pars.pt.creator.parse().map_err(Error::msg)?,
            shares_asset_id: pars.pt.shares_asset_id,
            invest_escrow: pars.pt.invest_escrow.try_into().map_err(Error::msg)?,
            locking_escrow: pars.pt.locking_escrow.try_into().map_err(Error::msg)?,
            central_escrow: pars.pt.central_escrow.try_into().map_err(Error::msg)?,
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
    pub pt: SubmitCreateDaoPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitCreateDaoPassthroughParJs {
    pub specs: CreateDaoSpecs,
    // not sure how to passthrough, if we use Address, when deserializing, we get:
    // index.js:1 Error("invalid type: sequence, expected a 32 byte array", line: 1, column: 10711)
    // looking at the logs, the passed JsValue looks like an array ([1, 2...])
    pub creator: String,
    // can't use SignedTransactions because of deserialization issue mainly (only?) with Address
    // see note on `creator` above
    // Note: multiple transactions: the tx vector is serialized into a single u8 vector
    pub escrow_optin_signed_txs_msg_pack: Vec<u8>,
    pub shares_asset_id: u64,
    pub invest_escrow: VersionedContractAccountJs,
    pub locking_escrow: VersionedContractAccountJs,
    pub central_escrow: VersionedContractAccountJs,
    pub customer_escrow: VersionedContractAccountJs,
    pub app_id: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitCreateDaoResJs {
    // next step tx: save the dao
    pub to_sign: Value,
}
