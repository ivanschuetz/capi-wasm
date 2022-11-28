use super::{
    create_dao_provider::{CreateDaoFormInputsJs, CreateDaoPassthroughParJs},
    providers,
};
use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, inputs_validation_js::ValidationErrorJs, to_sign_js::ToSignJs},
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateAssetsProvider {
    async fn txs(&self, pars: CreateDaoAssetsParJs) -> Result<CreateDaoAssetsResJs, FrError>;
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
    pub social_media_url: Option<ValidationErrorJs>,
    pub min_raise_target: Option<ValidationErrorJs>,
    pub min_raise_target_end_date: Option<ValidationErrorJs>,
    pub min_invest_shares: Option<ValidationErrorJs>,
    pub max_invest_shares: Option<ValidationErrorJs>,
    pub shares_for_investors: Option<ValidationErrorJs>,

    // note that these are not text inputs, but still, inputs
    pub image_url: Option<ValidationErrorJs>,
    pub prospectus_url: Option<ValidationErrorJs>,
    pub prospectus_bytes: Option<ValidationErrorJs>,
}

/// Specs to create assets (we need to sign this first, to get asset ids for the rest of the flow)
/// Note that asset price isn't here, as this is not needed/related to asset creation.
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CreateDaoAssetsParJs {
    pub inputs: CreateDaoFormInputsJs,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CreateDaoAssetsResJs {
    pub to_sign: ToSignJs,
    pub pt: CreateDaoPassthroughParJs, // passthrough
}

#[wasm_bindgen(js_name=createDaoAssetsTxs)]
pub async fn create_dao_assets_txs(
    pars: CreateDaoAssetsParJs,
) -> Result<CreateDaoAssetsResJs, FrError> {
    log_wrap_new("create_dao_assets", pars, async move |pars| {
        providers()?.create_assets.txs(pars).await
    })
    .await
}
