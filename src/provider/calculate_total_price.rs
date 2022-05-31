use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::def::calculate_total_price_def::ValidationCalcTotalPriceOrAnyhowError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CalculateTotalPriceProvider {
    async fn get(
        &self,
        pars: CalculateTotalPriceParJs,
    ) -> Result<CalculateTotalPriceResJs, ValidationCalcTotalPriceOrAnyhowError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculateTotalPriceParJs {
    pub shares_amount: String,
    pub available_shares: String,
    pub share_supply: String,
    pub investors_share: String,
    pub share_price: String,
    pub share_specs_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CalculateTotalPriceResJs {
    pub total_price: String,
    pub profit_percentage: String,
}
