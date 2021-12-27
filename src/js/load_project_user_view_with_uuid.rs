use crate::{
    dependencies::api,
    js::common::{parse_bridge_pars, to_bridge_res},
    server::api::Api,
    service::load_project_view_data::{
        load_project_for_users_view_data_with_project, ProjectForUsersViewData,
    },
};
use algonaut::algod::v2::Algod;
use anyhow::Result;
use core::dependencies::algod;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_project_user_view_with_uuid(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_project_user_view_with_uuid, pars: {:?}", pars);

    to_bridge_res(_bridge_load_project_user_view_with_uuid(parse_bridge_pars(pars)?).await)
}

async fn _bridge_load_project_user_view_with_uuid(
    project_uuid: String,
) -> Result<ProjectForUsersViewData> {
    let api = api();
    let algod = algod();

    Ok(load_project_view_data(&api, &algod, project_uuid).await?)
}

pub async fn load_project_view_data(
    api: &Api,
    algod: &Algod,
    project_uuid: String,
) -> Result<ProjectForUsersViewData> {
    log::debug!("load_project_view_data, uuid: {:?}", project_uuid);

    let project = api.load_project_user_view_with_uuid(&project_uuid).await?;
    load_project_for_users_view_data_with_project(algod, &project).await
}
