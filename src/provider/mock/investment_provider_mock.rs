use super::req_delay;
use crate::provider::investment_provider::{
    AvailableSharesParJs, AvailableSharesResJs, InvestmentProvider, LoadInvestorParJs,
    LoadInvestorResJs,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct InvestmentProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl InvestmentProvider for InvestmentProviderMock {
    async fn available_shares(&self, _pars: AvailableSharesParJs) -> Result<AvailableSharesResJs> {
        req_delay().await;

        Ok(AvailableSharesResJs {
            available_shares: "1000".to_owned(),
        })
    }

    async fn get_investor_data(&self, _: LoadInvestorParJs) -> Result<LoadInvestorResJs> {
        req_delay().await;

        Ok(LoadInvestorResJs {
            investor_shares_count: "123".to_owned(),

            investor_percentage: "21 %".to_owned(),
            investor_percentage_number: "21".to_owned(),

            investor_share: "12 %".to_owned(),

            investor_already_retrieved_amount: "11100".to_owned(),
            investor_claimable_dividend: "240".to_owned(),
            investor_claimable_dividend_microalgos: "0000".to_owned(),

            investor_locked_shares: "20".to_owned(),
            investor_unlocked_shares: "10".to_owned(),
        })
    }
}
