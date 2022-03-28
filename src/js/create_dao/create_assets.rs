use super::create_dao::{CreateDaoFormInputsJs, CreateDaoPassthroughParJs};
use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_bridge_res, to_my_algo_txs1};
use crate::js::create_dao::create_dao::validate_dao_inputs;
use crate::service::constants::PRECISION;
use anyhow::{Error, Result};
use core::api::api::Api;
use core::api::contract::Contract;
use core::dependencies::algod;
use core::flows::create_dao::setup::create_shares::create_assets;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// asset specs -> create assets txs
#[wasm_bindgen]
pub async fn bridge_create_dao_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_dao_assets, pars: {:?}", pars);
    to_bridge_res(_bridge_create_dao_assets_txs(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_create_dao_assets_txs(
    pars: CreateDaoAssetsParJs,
) -> Result<CreateDaoAssetsResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;
    let funds_asset_specs = funds_asset_specs()?;

    let dao_specs = pars.inputs.to_dao_specs(&funds_asset_specs)?;

    let validated_inputs = validate_dao_inputs(&pars.inputs, &funds_asset_specs)?;

    let last_versions = api.last_versions();
    let last_approval_tmpl = api.template(Contract::DaoAppApproval, last_versions.app_approval)?;
    let last_clear_tmpl = api.template(Contract::DaoAppClear, last_versions.app_clear)?;

    let create_assets_txs = create_assets(
        &algod,
        &validated_inputs.creator,
        &validated_inputs.creator, // for now creator is owner
        &dao_specs,
        &last_approval_tmpl,
        &last_clear_tmpl,
        PRECISION,
        &capi_deps,
    )
    .await?;

    Ok(CreateDaoAssetsResJs {
        to_sign: to_my_algo_txs1(&[
            create_assets_txs.create_shares_tx,
            create_assets_txs.create_app_tx,
        ])
        .map_err(Error::msg)?,
        // we forward the inputs to the next step, just for a little convenience (javascript could pass them as separate fields again instead)
        // the next step will validate them again, as this performs type conversion too (+ general safety)
        pt: CreateDaoPassthroughParJs {
            inputs: pars.inputs,
        },
    })
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDaoAssetsParJs {
    pub inputs: CreateDaoFormInputsJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDaoAssetsResJs {
    pub to_sign: Vec<Value>,
    pub pt: CreateDaoPassthroughParJs, // passthrough
}
