use crate::js::common::SignedTxFromJs;
use crate::js::common::{parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res};
use crate::model::project_for_users::project_to_project_for_users;
use crate::model::project_for_users_view_data::ProjectForUsersViewData;
use anyhow::Result;
use core::dependencies::algod;
use core::flows::create_project::storage::load_project::ProjectId;
use core::flows::create_project::storage::save_project::{submit_save_project, SaveProjectSigned};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_submit_save_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_save_project, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_save_project(parse_bridge_pars(pars)?).await)
}

/// create projects specs + signed assets txs -> create project result
/// submits the signed assets, creates rest of project with generated asset ids
async fn _bridge_submit_save_project(
    pars: SubmitSaveProjectParJs,
) -> Result<ProjectForUsersViewData> {
    let algod = algod();

    let project = rmp_serde::from_slice(&pars.pt.project_msg_pack)?;

    let tx_id = submit_save_project(
        &algod,
        SaveProjectSigned {
            tx: signed_js_tx_to_signed_tx1(&pars.tx)?,
        },
    )
    .await?;

    log::debug!("Save project tx id: {:?}", tx_id);

    let project_id = ProjectId(tx_id);

    // TODO better typing: if we keep the tx id as project id, we should create a new tx_id type (which would also contain the hash digest conversion)
    Ok(project_to_project_for_users(&project, &project_id)?.into())
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitSaveProjectParJs {
    pub tx: SignedTxFromJs,                    // store project signed transaction
    pub pt: SubmitSaveProjectPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSaveProjectPassthroughParJs {
    pub project_msg_pack: Vec<u8>,
}
