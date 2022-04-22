use super::req_delay;
use crate::provider::investment_provider::{
    InvestmentProvider, LoadInvestmentParJs, LoadInvestmentResJs,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct InvestmentProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl InvestmentProvider for InvestmentProviderMock {
    async fn get(&self, _: LoadInvestmentParJs) -> Result<LoadInvestmentResJs> {
        req_delay().await;

        Ok(LoadInvestmentResJs {
            investor_shares_count: "123".to_owned(),

            investor_percentage: "21 %".to_owned(),
            investor_percentage_number: "21".to_owned(),
            investor_percentage_relative_to_total_number: "12".to_owned(),
            investors_share_number: "0.2".to_owned(),

            investor_already_retrieved_amount: "11100".to_owned(),
            investor_claimable_dividend: "240".to_owned(),
            investor_claimable_dividend_microalgos: "0000".to_owned(),
        })
    }
}
