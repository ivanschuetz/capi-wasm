use crate::{
    provider::{
        calculate_total_price::{
            CalculateTotalPriceParJs, CalculateTotalPriceProvider, CalculateTotalPriceResJs,
        },
        def::calculate_total_price_def::ValidationCalcTotalPriceOrAnyhowError,
    },
    service::number_formats::format_u64_readable,
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
            total_price: format_u64_readable(12345)?,
            profit_percentage: "0.23 %".to_owned(),
        })
    }
}
