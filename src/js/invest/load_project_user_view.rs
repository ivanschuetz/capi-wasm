use crate::{
    dependencies::api,
    js::common::{parse_bridge_pars, to_bridge_res},
    service::load_project_view_data::{load_project_view_data, ProjectForUsersViewData},
};
use anyhow::Result;
use core::dependencies::algod;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_project_user_view(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_project_user_view, pars: {:?}", pars);

    to_bridge_res(_bridge_load_project_user_view(parse_bridge_pars(pars)?).await)
}

async fn _bridge_load_project_user_view(project_id: String) -> Result<ProjectForUsersViewData> {
    log::debug!("load_project, id: {:?}", project_id);

    let api = api();
    let algod = algod();

    Ok(load_project_view_data(&api, &algod, project_id).await?)
}
