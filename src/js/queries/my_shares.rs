use crate::{
    js::common::{parse_bridge_pars, to_bridge_res},
    teal::programs,
};
use anyhow::{Error, Result};
use core::{
    dependencies::{algod, indexer},
    flows::create_project::storage::load_project::load_project,
    state::{app_state::ApplicationLocalStateError, central_app_state::central_investor_state},
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
    let indexer = indexer();

    let project_id = pars.project_id.parse()?;

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows)
        .await?
        .project;

    log::debug!("Project: {project:?}");

    let my_address = &pars.my_address.parse().map_err(Error::msg)?;

    let my_shares_str =
        match central_investor_state(&algod, my_address, project.central_app_id).await {
            Ok(state) => state.shares.to_string(),
            Err(ApplicationLocalStateError::NotOptedIn) => "0".to_owned(), // not invested -> 0 shares
            Err(e) => return Err(Error::msg(e)),
        };

    Ok(MySharesResJs {
        shares: my_shares_str,
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct MySharesParJs {
    pub project_id: String,
    pub my_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MySharesResJs {
    pub shares: String,
}
