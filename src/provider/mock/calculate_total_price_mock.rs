use crate::provider::{
    calculate_total_price::{
        CalculateTotalPriceParJs, CalculateTotalPriceProvider, CalculateTotalPriceResJs,
    },
    def::calculate_total_price_def::ValidationCalcTotalPriceOrAnyhowError,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct CalculateTotalPriceMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CalculateTotalPriceProvider for CalculateTotalPriceMock {
    async fn get(
        &self,
        _: CalculateTotalPriceParJs,
    ) -> Result<CalculateTotalPriceResJs, ValidationCalcTotalPriceOrAnyhowError> {
        Ok(CalculateTotalPriceResJs {
            total_price: "12345".to_owned(),
            profit_percentage: "0.23 %".to_owned(),
        })
    }
}
