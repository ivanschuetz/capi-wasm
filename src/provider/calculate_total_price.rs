use crate::error::FrError;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CalculateTotalPriceProvider {
    async fn get(
        &self,
        pars: CalculateTotalPriceParJs,
    ) -> Result<CalculateTotalPriceResJs, FrError>;

    async fn max_funds(
        &self,
        pars: CalculateMaxFundsParJs,
    ) -> Result<CalculateMaxFundsResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculateTotalPriceParJs {
    pub shares_amount: String,
    pub available_shares: String,
    pub share_supply: String,
    pub locked_shares: Option<String>,
    pub investors_share: String,
    pub share_price: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CalculateTotalPriceResJs {
    pub total_price: String,
    pub total_price_number: String,
    pub profit_percentage: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalculateMaxFundsParJs {
    pub shares_amount: String,
    pub share_price: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CalculateMaxFundsResJs {
    pub total_price: String,
    pub total_price_number: String,
}
