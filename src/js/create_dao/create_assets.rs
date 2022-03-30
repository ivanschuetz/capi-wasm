use super::create_dao::{CreateDaoFormInputsJs, CreateDaoPassthroughParJs, ValidateDaoInputsError};
use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::{parse_bridge_pars, to_js_value, to_my_algo_txs1};
use crate::js::create_dao::create_dao::validate_dao_inputs;
use crate::js::inputs_validation_js::{to_validation_error_js, ValidationErrorJs};
use crate::service::constants::PRECISION;
use algonaut::core::Address;
use anyhow::{Error, Result};
use core::api::api::Api;
use core::api::contract::Contract;
use core::dependencies::algod;
use core::flows::create_dao::create_dao_specs::CreateDaoSpecs;
use core::flows::create_dao::setup::create_shares::create_assets;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// asset specs -> create assets txs
#[wasm_bindgen]
pub async fn bridge_create_dao_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_dao_assets, pars: {:?}", pars);
    _bridge_create_dao_assets_txs(parse_bridge_pars(pars)?)
        .await
        .map(to_js_value)
}

pub async fn _bridge_create_dao_assets_txs(
    pars: CreateDaoAssetsParJs,
) -> Result<CreateDaoAssetsResJs, JsValue> {
    let funds_asset_specs = funds_asset_specs().map_err(to_js_value)?;

    // Note: partly redundant validation here (to_dao_specs validates everything again)
    let validated_inputs = validate_dao_inputs(&pars.inputs, &funds_asset_specs)?;
    let dao_specs = pars.inputs.to_dao_specs(&funds_asset_specs)?;

    create_dao_assets_txs(&dao_specs, &validated_inputs.creator, pars.inputs)
        .await
        .map_err(to_js_value)
}

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateAssetsInputErrorsJs {
    // just some string to identify the struct in js
    pub type_identifier: String,
    pub name: Option<ValidationErrorJs>,
    pub description: Option<ValidationErrorJs>,
    pub creator: Option<ValidationErrorJs>,
    pub share_supply: Option<ValidationErrorJs>,
    pub share_price: Option<ValidationErrorJs>,
    pub investors_share: Option<ValidationErrorJs>,
    pub logo_url: Option<ValidationErrorJs>,
    pub social_media_url: Option<ValidationErrorJs>,
}

impl From<ValidateDaoInputsError> for JsValue {
    fn from(error: ValidateDaoInputsError) -> JsValue {
        match error {
            ValidateDaoInputsError::AllFieldsValidation(e) => {
                let errors_js = CreateAssetsInputErrorsJs {
                    type_identifier: "input_errors".to_owned(),
                    name: e.name.map(to_validation_error_js),
                    description: e.description.map(to_validation_error_js),
                    creator: e.creator.map(to_validation_error_js),
                    share_supply: e.share_supply.map(to_validation_error_js),
                    share_price: e.share_price.map(to_validation_error_js),
                    investors_share: e.investors_share.map(to_validation_error_js),
                    logo_url: e.logo_url.map(to_validation_error_js),
                    social_media_url: e.social_media_url.map(to_validation_error_js),
                };
                match JsValue::from_serde(&errors_js) {
                    Ok(js) => js,
                    Err(e) => to_js_value(e),
                }
            }
            _ => to_js_value(format!("Error processing inputs: {error:?}")),
        }
    }
}

async fn create_dao_assets_txs(
    dao_specs: &CreateDaoSpecs,
    creator: &Address,
    inputs: CreateDaoFormInputsJs,
) -> Result<CreateDaoAssetsResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    let last_versions = api.last_versions();
    let last_approval_tmpl = api.template(Contract::DaoAppApproval, last_versions.app_approval)?;
    let last_clear_tmpl = api.template(Contract::DaoAppClear, last_versions.app_clear)?;

    let create_assets_txs = create_assets(
        &algod,
        creator,
        creator, // for now creator is owner
        dao_specs,
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
        pt: CreateDaoPassthroughParJs { inputs },
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

pub struct CreateDaoValidationErrors {}
