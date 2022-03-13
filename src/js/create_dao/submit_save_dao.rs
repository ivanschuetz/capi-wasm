use crate::dependencies::funds_asset_specs;
use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::model::dao_for_users::dao_to_dao_for_users;
use crate::model::dao_for_users_view_data::{
    dao_for_users_to_view_data, DaoForUsersViewData,
};
use anyhow::Result;
use core::dependencies::algod;
use core::flows::create_dao::storage::load_dao::DaoId;
use core::flows::create_dao::storage::save_dao::{submit_save_dao, SaveDaoSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_save_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_save_dao, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_save_dao(parse_bridge_pars(pars)?).await)
}

/// create daos specs + signed assets txs -> create dao result
/// submits the signed assets, creates rest of dao with generated asset ids
async fn _bridge_submit_save_dao(
    pars: SubmitSaveDaoParJs,
) -> Result<DaoForUsersViewData> {
    let algod = algod();

    let dao = rmp_serde::from_slice(&pars.pt.dao_msg_pack)?;

    let tx_id = submit_save_dao(
        &algod,
        SaveDaoSigned {
            tx: signed_js_tx_to_signed_tx1(&pars.tx)?,
        },
    )
    .await?;

    log::debug!("Save dao tx id: {:?}", tx_id);

    let dao_id = DaoId(tx_id);

    // TODO better typing: if we keep the tx id as dao id, we should create a new tx_id type (which would also contain the hash digest conversion)
    Ok(dao_for_users_to_view_data(
        dao_to_dao_for_users(&dao, &dao_id)?,
        &funds_asset_specs(),
    ))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitSaveDaoParJs {
    pub tx: SignedTxFromJs,                    // store dao signed transaction
    pub pt: SubmitSaveDaoPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSaveDaoPassthroughParJs {
    pub dao_msg_pack: Vec<u8>,
}
