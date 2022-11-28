use crate::{error::FrError, js::bridge::log_wrap_new, provider::providers};
use data_encoding::BASE64;
use mbase::models::hashable::hash;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct HashProviderDef {}

impl HashProviderDef {
    pub async fn hash(&self, pars: HashPars) -> Result<String, FrError> {
        Ok(BASE64.encode(&hash(&pars.bytes).0))
    }
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct HashPars {
    bytes: Vec<u8>,
}

#[wasm_bindgen(js_name=calculateHash)]
pub async fn calculate_hash(pars: HashPars) -> Result<String, FrError> {
    log_wrap_new("calculate_hash", pars, async move |pars| {
        providers()?.hash.hash(pars).await
    })
    .await
}
