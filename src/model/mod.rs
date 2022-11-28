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
