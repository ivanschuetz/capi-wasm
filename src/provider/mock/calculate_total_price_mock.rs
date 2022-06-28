use crate::{
    error::FrError,
    provider::calculate_total_price::{
        CalculateMaxFundsParJs, CalculateMaxFundsResJs, CalculateTotalPriceParJs,
        CalculateTotalPriceProvider, CalculateTotalPriceResJs,
    },
    service::number_formats::format_u64_readable,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct CalculateTotalPriceMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CalculateTotalPriceProvider for CalculateTotalPriceMock {
    async fn get(&self, _: CalculateTotalPriceParJs) -> Result<CalculateTotalPriceResJs, FrError> {
        Ok(CalculateTotalPriceResJs {
            total_price: format_u64_readable(12345)?,
            total_price_number: "12345".to_owned(),
            profit_percentage: "0.23 %".to_owned(),
        })
    }

    async fn max_funds(
        &self,
        _pars: CalculateMaxFundsParJs,
    ) -> Result<CalculateMaxFundsResJs, FrError> {
        Ok(CalculateMaxFundsResJs {
            total_price: format_u64_readable(12345)?,
            total_price_number: "12345".to_owned(),
        })
    }
}
