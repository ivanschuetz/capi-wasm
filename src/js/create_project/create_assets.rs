use super::create_project::{CreateProjectFormInputsJs, CreateProjectPassthroughParJs};
use crate::dependencies::funds_asset_specs;
use crate::js::common::{to_js_value, to_my_algo_tx};
use crate::js::create_project::create_project::validate_project_inputs;
use core::dependencies::algod;
use core::flows::create_project::{
    model::CreateSharesSpecs, setup::create_assets::create_investor_assets_txs,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// asset specs -> create assets txs
#[wasm_bindgen]
pub async fn bridge_create_project_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    let algod = algod();

    let pars = pars
        .into_serde::<CreateProjectAssetsParJs>()
        .map_err(to_js_value)?;

    let validated_inputs =
        validate_project_inputs(&pars.inputs, &funds_asset_specs()).map_err(to_js_value)?;

    let create_assets_txs = create_investor_assets_txs(
        &algod,
        &validated_inputs.creator,
        &CreateSharesSpecs {
            token_name: validated_inputs.token_name,
            count: validated_inputs.share_count,
        },
    )
    .await
    .map_err(to_js_value)?;

    let res = CreateProjectAssetsResJs {
        to_sign: vec![to_my_algo_tx(&create_assets_txs.create_shares_tx)?],
        // we forward the inputs to the next step, just for a little convenience (javascript could pass them as separate fields again instead)
        // the next step will validate them again, as this performs type conversion too (+ general safety)
        pt: CreateProjectPassthroughParJs {
            inputs: pars.inputs,
        },
    };

    Ok(JsValue::from_serde(&res).map_err(to_js_value)?)
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
