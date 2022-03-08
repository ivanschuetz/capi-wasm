use super::create_project::{CreateProjectFormInputsJs, CreateProjectPassthroughParJs};
use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use crate::js::create_project::create_project::validate_project_inputs;
use crate::service::constants::PRECISION;
use crate::teal;
use anyhow::{Error, Result};
use core::dependencies::algod;
use core::flows::create_project::setup::create_shares::create_assets;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// asset specs -> create assets txs
#[wasm_bindgen]
pub async fn bridge_create_project_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_project_assets, pars: {:?}", pars);
    to_bridge_res(_bridge_create_project_assets_txs(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_create_project_assets_txs(
    pars: CreateProjectAssetsParJs,
) -> Result<CreateProjectAssetsResJs> {
    let algod = algod();
    let capi_deps = capi_deps()?;
    let funds_asset_specs = funds_asset_specs();

    let project_specs = pars.inputs.to_project_specs(&funds_asset_specs)?;

    let validated_inputs = validate_project_inputs(&pars.inputs, &funds_asset_specs)?;

    let create_assets_txs = create_assets(
        &algod,
        &validated_inputs.creator,
        &project_specs,
        &teal::programs(),
        PRECISION,
        &capi_deps,
    )
    .await?;

    Ok(CreateProjectAssetsResJs {
        to_sign: to_my_algo_txs1(&[
            create_assets_txs.create_shares_tx,
            create_assets_txs.create_app_tx,
        ])
        .map_err(Error::msg)?,
        // we forward the inputs to the next step, just for a little convenience (javascript could pass them as separate fields again instead)
        // the next step will validate them again, as this performs type conversion too (+ general safety)
        pt: CreateProjectPassthroughParJs {
            inputs: pars.inputs,
        },
    })
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectAssetsParJs {
    pub inputs: CreateProjectFormInputsJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProjectAssetsResJs {
    pub to_sign: Vec<Value>,
    pub pt: CreateProjectPassthroughParJs, // passthrough
}
