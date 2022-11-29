use mbase::state::dao_app_state::Prospectus;
use serde::Serialize;
use tsify::Tsify;

pub mod dao_js;

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub enum QuantityChangeJs {
    Up,
    Down,
    Eq,
    Unknown,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ProspectusJs {
    pub hash: String,
    pub url: String,
}

impl From<Prospectus> for ProspectusJs {
    fn from(p: Prospectus) -> Self {
        ProspectusJs {
            hash: p.hash,
            url: p.url,
        }
    }
}
