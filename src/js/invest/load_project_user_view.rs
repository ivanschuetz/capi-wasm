use crate::{
    dependencies::funds_asset_specs,
    js::common::{parse_bridge_pars, to_bridge_res},
    model::{
        project_for_users::project_to_project_for_users,
        project_for_users_view_data::{project_for_users_to_view_data, ProjectForUsersViewData},
    },
    teal::programs,
};
use anyhow::Result;
use core::{
    dependencies::{algod, indexer},
    flows::create_project::storage::load_project::load_project,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_project_user_view(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_project_user_view, pars: {:?}", pars);

    to_bridge_res(_bridge_load_project_user_view(parse_bridge_pars(pars)?).await)
}

async fn _bridge_load_project_user_view(project_id_str: String) -> Result<ProjectForUsersViewData> {
    log::debug!("load_project, hash: {:?}", project_id_str);

    let algod = algod();
    let indexer = indexer();

    let project_id = project_id_str.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows)
        .await?
        .project;

    Ok(project_for_users_to_view_data(
        project_to_project_for_users(&project, &project_id)?,
        &funds_asset_specs(),
    ))
}
