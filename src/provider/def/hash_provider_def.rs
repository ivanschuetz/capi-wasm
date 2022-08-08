use crate::error::FrError;
use base::hashable::hash;
use data_encoding::BASE64;
use serde::Deserialize;

pub struct HashProviderDef {}

impl HashProviderDef {
    pub async fn hash(&self, pars: HashPars) -> Result<String, FrError> {
        Ok(BASE64.encode(&hash(&pars.bytes).0))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HashPars {
    bytes: Vec<u8>,
}
