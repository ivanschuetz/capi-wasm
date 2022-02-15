use anyhow::{anyhow, Error, Result};
use core::dependencies::algod;
use core::flows::create_project::create_project_specs::CreateProjectSpecs;
use core::flows::create_project::storage::save_project::save_project;
use core::flows::create_project::{
    create_project::submit_create_project,
    model::CreateProjectSigned,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, signed_js_txs_to_signed_tx1, to_bridge_res,
};
use crate::js::common::{to_my_algo_tx1, SignedTxFromJs};
use crate::js::general::js_types_workarounds::ContractAccountJs;

use super::submit_save_project::SubmitSaveProjectPassthroughParJs;

#[wasm_bindgen]
pub async fn bridge_submit_create_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_create_project, pars: {:?}", pars);
    to_bridge_res(_bridge_submit_create_project(parse_bridge_pars(pars)?).await)
}

/// create projects specs + signed assets txs -> create project result
/// submits the signed assets, creates rest of project with generated asset ids
async fn _bridge_submit_create_project(
    pars: SubmitCreateProjectParJs,
) -> Result<SubmitCreateProjectResJs> {
    // log::debug!("in bridge_submit_create_project, pars: {:?}", pars);

    let algod = algod();

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
    let create_app_tx = &pars.txs[0];
    let escrow_funding_txs = &pars.txs[1..5];
    let xfer_shares_to_invest_escrow = &pars.txs[5];

    let project_creator = pars.pt.creator.parse().map_err(Error::msg)?;

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

    // let save_project_res = api.save_project(&submit_project_res.project).await?;

    let to_sign = save_project(&algod, &project_creator, &submit_project_res.project).await?;

    Ok(SubmitCreateProjectResJs {
        to_sign: to_my_algo_tx1(&to_sign.tx)?,
        pt: SubmitSaveProjectPassthroughParJs {
            project_msg_pack: rmp_serde::to_vec_named(&to_sign.project)?,
        },
    })
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
    pub invest_escrow: ContractAccountJs,
    pub staking_escrow: ContractAccountJs,
    pub central_escrow: ContractAccountJs,
    pub customer_escrow: ContractAccountJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitCreateProjectResJs {
    // next step tx: save the project
    pub to_sign: Value,
    pub pt: SubmitSaveProjectPassthroughParJs,
}
