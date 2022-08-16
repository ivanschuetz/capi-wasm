use crate::error::FrError;
use anyhow::Result;

pub struct MetadataProviderDef {}

impl MetadataProviderDef {
    pub fn wasm_version(&self) -> Result<String, FrError> {
        let version = env!("CARGO_PKG_VERSION");
        log::debug!("Returning wasm version: {version:?}");
        Ok(version.to_owned())
    }
}
