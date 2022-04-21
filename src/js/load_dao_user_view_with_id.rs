use crate::{
    dependencies::{api, capi_deps, funds_asset_specs},
    js::common::{parse_bridge_pars, to_bridge_res},
    model::{
        dao_for_users::dao_to_dao_for_users,
        dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData},
    },
};
use anyhow::Result;
use base::{dependencies::algod, flows::create_dao::storage::load_dao::load_dao};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_dao_user_view_with_id(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_dao_user_view_with_id, pars: {:?}", pars);
    to_bridge_res(_bridge_load_dao_user_view_with_id(parse_bridge_pars(pars)?).await)
}

async fn _bridge_load_dao_user_view_with_id(dao_id_str: String) -> Result<DaoForUsersViewData> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    let dao_id = dao_id_str.parse()?;

    let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

    Ok(dao_for_users_to_view_data(
        dao_to_dao_for_users(&dao, &dao_id)?,
        &funds_asset_specs()?,
    ))
}
