use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    js::common::{parse_bridge_pars, to_bridge_res},
    model::{
        dao_for_users::dao_to_dao_for_users,
        dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData},
    },
    teal::programs,
};
use anyhow::Result;
use core::{
    dependencies::{algod, indexer},
    flows::create_dao::storage::load_dao::load_dao,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_dao_user_view(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_dao_user_view, pars: {:?}", pars);

    to_bridge_res(_bridge_load_dao_user_view(parse_bridge_pars(pars)?).await)
}

async fn _bridge_load_dao_user_view(dao_id_str: String) -> Result<DaoForUsersViewData> {
    log::debug!("load_dao, hash: {:?}", dao_id_str);

    let algod = algod();
    let indexer = indexer();
    let capi_deps = capi_deps()?;
    let programs = programs();

    let dao_id = dao_id_str.parse()?;

    let dao = load_dao(&algod, &indexer, &dao_id, &programs.escrows, &capi_deps)
        .await?
        .dao;

    Ok(dao_for_users_to_view_data(
        dao_to_dao_for_users(&dao, &dao_id)?,
        &funds_asset_specs(),
    ))
}
