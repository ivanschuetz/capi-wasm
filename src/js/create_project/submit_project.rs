use anyhow::{anyhow, Error, Result};
use core::api::json_workaround::ContractAccountJson;
use core::flows::create_project::{
    logic::submit_create_project,
    model::{CreateProjectSigned, CreateProjectSpecs, Project},
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

use crate::dependencies::environment;
use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, signed_js_txs_to_signed_tx1, to_bridge_res,
};
use crate::service::load_project_view_data::{
    project_for_users_to_view_data, ProjectForUsersViewData,
};
use crate::{
    dependencies::{algod, api},
    js::common::SignedTxFromJs,
};

#[wasm_bindgen]
pub async fn bridge_submit_create_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_create_project, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_create_project(parse_bridge_pars(pars)?).await)
}

/// create projects specs + signed assets txs -> create project result
/// submits the signed assets, creates rest of project with generated asset ids
async fn _bridge_submit_create_project(
    pars: SubmitCreateProjectParJs,
) -> Result<ProjectForUsersViewData> {
    // log::debug!("in bridge_submit_create_project, pars: {:?}", pars);

    let env = &environment();
    let algod = algod(env);
    let api = api(env);

    if pars.txs.len() != 6 {
        return Err(anyhow!(
            "Unexpected signed project txs length: {}",
            pars.txs.len()
        ));
    }

    // TODO (low prio) improve this access, it's easy for the indices to get out of sync
    // and assign the txs to incorrect variables, which may cause subtle bugs
    // maybe refactor writing/reading into a helper struct or function
    // (written in create_project::txs_to_sign)
    let escrow_funding_txs = &pars.txs[0..4];
    let create_app_tx = &pars.txs[4];
    let xfer_shares_to_invest_escrow = &pars.txs[5];

    log::debug!("Submitting the project..");

    let submit_project_res = submit_create_project(
        &algod,
        CreateProjectSigned {
            escrow_funding_txs: signed_js_txs_to_signed_tx1(escrow_funding_txs)?,
            optin_txs: rmp_serde::from_slice(&pars.pt.escrow_optin_signed_txs_msg_pack)
                .map_err(Error::msg)?,
            create_app_tx: signed_js_tx_to_signed_tx1(create_app_tx)?,
            xfer_shares_to_invest_escrow: signed_js_tx_to_signed_tx1(xfer_shares_to_invest_escrow)?,
            specs: pars.pt.specs,
            creator: pars.pt.creator.parse().map_err(Error::msg)?,
            shares_asset_id: pars.pt.shares_asset_id,
            invest_escrow: pars.pt.invest_escrow.try_into().map_err(Error::msg)?,
            staking_escrow: pars.pt.staking_escrow.try_into().map_err(Error::msg)?,
            central_escrow: pars.pt.central_escrow.try_into().map_err(Error::msg)?,
            customer_escrow: pars.pt.customer_escrow.try_into().map_err(Error::msg)?,
        },
    )
    .await?;

    log::debug!("Submit project res: {:?}", submit_project_res);

    let save_project_res = api.save_project(&submit_project_res.project).await?;

    Ok(project_for_users_to_view_data(
        save_project_res,
        // project was just created: share supply is what was entered as share count
        // (in the future share count can change with dilution)
        submit_project_res.project.specs.shares.count,
        // same here: freshly created, so the name is what was just entered
        submit_project_res.project.specs.shares.token_name,
    ))
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitCreateProjectParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitCreateProjectPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitCreateProjectPassthroughParJs {
    pub specs: CreateProjectSpecs,
    // not sure how to passthrough, if we use Address, when deserializing, we get:
    // index.js:1 Error("invalid type: sequence, expected a 32 byte array", line: 1, column: 10711)
    // looking at the logs, the passed JsValue looks like an array ([1, 2...])
    pub creator: String,
    // can't use SignedTransactions because of deserialization issue mainly (only?) with Address
    // see note on `creator` above
    // Note: multiple transactions: the tx vector is serialized into a single u8 vector
    pub escrow_optin_signed_txs_msg_pack: Vec<u8>,
    pub shares_asset_id: u64,
    pub invest_escrow: ContractAccountJson,
    pub staking_escrow: ContractAccountJson,
    pub central_escrow: ContractAccountJson,
    pub customer_escrow: ContractAccountJson,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitCreateProjectResJs {
    pub project: Project,
}
