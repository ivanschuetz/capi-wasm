use crate::{error::FrError, js::bridge::log_wrap_new_sync_no_pars, provider::providers};
use anyhow::Result;
use wasm_bindgen::prelude::wasm_bindgen;

pub struct MetadataProviderDef {}

impl MetadataProviderDef {
    pub fn wasm_version(&self) -> Result<String, FrError> {
        let version = env!("CARGO_PKG_VERSION");
        log::debug!("Returning wasm version: {version:?}");
        Ok(version.to_owned())
    }
}

#[wasm_bindgen(js_name=wasmVersion)]
pub async fn wasm_version() -> Result<String, FrError> {
    log_wrap_new_sync_no_pars("wasm_version", move || providers()?.metadata.wasm_version()).await
}
