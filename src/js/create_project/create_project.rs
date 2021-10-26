use algonaut::{core::Address, transaction::Transaction};
use make::api::model::DefaultError;
use make::flows::create_project::{
    logic::create_project_txs,
    model::{CreateProjectSpecs, CreateProjectToSign, CreateSharesSpecs},
    setup::create_assets::submit_create_assets,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

use wasm_bindgen::prelude::*;

use crate::dependencies::environment;
use crate::service::constants::{PRECISION, WITHDRAWAL_SLOT_COUNT};
use crate::{
    dependencies::algod,
    js::common::{signed_js_tx_to_signed_tx, to_js_value, to_my_algo_txs, SignedTxFromJs},
    server::api,
    service::str_to_algos::algos_str_to_microalgos,
};

use super::{
    create_assets::CreateProjectAssetsParJs, submit_project::SubmitCreateProjectPassthroughParJs,
};

/// create projects specs + signed assets txs -> create project result
/// submits the signed assets, creates rest of project with generated asset ids
#[wasm_bindgen]
pub async fn bridge_create_project(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_project, pars: {:?}", pars);

    let algod = algod(&environment());

    let pars = pars
        .into_serde::<CreateProjectParJs>()
        .map_err(to_js_value)?;

    // we assume order: js has as little logic as possible:
    // we send txs to be signed, as an array, and get the signed txs array back
    // js doesn't access the individual array txs, just passes the array to myalgo and gets signed array back
    // so this is the order in which we sent the txs to be signed, from the previously called rust fn.
    let create_shares_signed_tx = &pars.create_assets_signed_txs[0];

    let submit_assets_res = submit_create_assets(
        &algod,
        &signed_js_tx_to_signed_tx(&create_shares_signed_tx)?,
    )
    .await
    .map_err(to_js_value)?;

    let creator_address: Address = pars.creator.parse()?;

    let to_sign = create_project_txs(
        &algod,
        &pars.project_specs().map_err(to_js_value)?,
        creator_address,
        submit_assets_res.shares_id,
        api::programs().map_err(to_js_value)?,
        WITHDRAWAL_SLOT_COUNT,
        PRECISION,
    )
    .await
    .map_err(to_js_value)?;

    // since we've to bundle all the txs to be signed in one array (so the user has to confirm only once in myalgo)
    // but return the functions in separate groups to the core logic (so rely on indices),
    // (separate groups are needed since groups need to be executed in specific order, e.g. opt in before transferring assets)
    // we double-check length here. The other txs to be signed are in single tx fields so no need to check those.
    if to_sign.escrow_funding_txs.len() != 4 {
        return Err(JsValue::from_str(&format!(
            "Unexpected funding txs length: {}",
            to_sign.escrow_funding_txs.len()
        )));
    }
    // double-checking total length as well, just in case
    // in the next step we also check the length of the signed txs
    let txs_to_sign = &txs_to_sign(&to_sign);
    if txs_to_sign.len() as u64 != 6 + WITHDRAWAL_SLOT_COUNT {
        return Err(JsValue::from_str(&format!(
            "Unexpected to sign project txs length: {}",
            txs_to_sign.len()
        )));
    }

    let res = CreateProjectResJs {
        to_sign: to_my_algo_txs(txs_to_sign)?,
        pt: SubmitCreateProjectPassthroughParJs {
            specs: pars.project_specs().map_err(to_js_value)?,
            creator: creator_address.to_string(),
            escrow_optin_signed_txs_msg_pack: rmp_serde::to_vec_named(&to_sign.optin_txs)
                .map_err(to_js_value)?,
            shares_asset_id: submit_assets_res.shares_id,
            invest_escrow: to_sign.invest_escrow.into(),
            staking_escrow: to_sign.staking_escrow.into(),
            central_escrow: to_sign.central_escrow.into(),
            customer_escrow: to_sign.customer_escrow.into(),
        },
    };

    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
}

fn txs_to_sign(res: &CreateProjectToSign) -> Vec<Transaction> {
    let mut txs = vec![];
    for tx in &res.escrow_funding_txs {
        txs.push(tx.to_owned());
    }
    txs.push(res.create_app_tx.clone());
    txs.push(res.xfer_shares_to_invest_escrow.clone());
    for tx in &res.create_withdrawal_slots_txs {
        txs.push(tx.to_owned());
    }
    txs
}

/// The assets creation signed transactions and the specs to create the project
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectParJs {
    pub name: String,
    pub creator: String,
    pub asset_specs: CreateProjectAssetsParJs,
    pub asset_price: String,
    pub vote_threshold: String,
    // same order as the unsigned txs were sent to JS
    pub create_assets_signed_txs: Vec<SignedTxFromJs>,
}

impl CreateProjectParJs {
    fn project_specs(&self) -> Result<CreateProjectSpecs, DefaultError> {
        Ok(CreateProjectSpecs {
            name: self.name.clone(),
            shares: CreateSharesSpecs {
                token_name: self.asset_specs.token_name.clone(),
                count: self.asset_specs.count.parse()?,
            },
            asset_price: algos_str_to_microalgos(&self.asset_price)?,
            vote_threshold: self.vote_threshold.parse()?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProjectResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitCreateProjectPassthroughParJs, // passthrough
}
