use make::api::model::DefaultError;
use make::flows::create_project::{
    model::CreateSharesSpecs, setup::create_assets::create_investor_assets_txs,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

use wasm_bindgen::prelude::*;

use crate::service::str_to_algos::algos_str_to_microalgos;
use crate::{
    dependencies::{algod, environment},
    js::common::{to_js_value, to_my_algo_tx},
};

/// asset specs -> create assets txs
#[wasm_bindgen]
pub async fn bridge_create_project_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    let algod = algod(&environment());

    let pars = pars
        .into_serde::<CreateProjectAssetsParJs>()
        .map_err(to_js_value)?;

    // validate the price input. we don't use it in this step,
    // just so the user sees possible errors right away, not after signing the asset txs
    algos_str_to_microalgos(&pars.asset_price).map_err(to_js_value)?;

    let create_assets_txs = create_investor_assets_txs(
        &algod,
        &pars.creator.parse().map_err(to_js_value)?,
        &pars.clone().try_into().map_err(to_js_value)?,
    )
    .await
    .map_err(to_js_value)?;

    let tnxs_for_js = CreateProjectAssetsResJs {
        to_sign: vec![to_my_algo_tx(&create_assets_txs.create_shares_tx)?],
        asset_spec: pars,
    };

    Ok(JsValue::from_serde(&tnxs_for_js).map_err(to_js_value)?)
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectAssetsParJs {
    pub creator: String,
    pub token_name: String,
    pub count: String,
    pub asset_price: String, // only for validation
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProjectAssetsResJs {
    pub to_sign: Vec<Value>,
    pub asset_spec: CreateProjectAssetsParJs, // passthrough
}

// not sure this makes a lot of sense, in place conversion may be better, for now like this
// one thing were this is better is to not have to convert every field to error
// but we want to refactor that either way, alternatively simple mapping functions might be fine too
impl TryFrom<CreateProjectAssetsParJs> for CreateSharesSpecs {
    type Error = DefaultError;

    fn try_from(js: CreateProjectAssetsParJs) -> Result<Self, Self::Error> {
        Ok(CreateSharesSpecs {
            token_name: js.token_name.clone(),
            count: js.count.parse()?,
        })
    }
}
