use super::req_delay;
use crate::{
    error::FrError,
    provider::investment_provider::{
        AvailableSharesParJs, AvailableSharesResJs, InvestmentProvider, LoadInvestorParJs,
        LoadInvestorResJs,
    },
};
use anyhow::Result;
use async_trait::async_trait;

pub struct InvestmentProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl InvestmentProvider for InvestmentProviderMock {
    async fn available_shares(
        &self,
        _pars: AvailableSharesParJs,
    ) -> Result<AvailableSharesResJs, FrError> {
        req_delay().await;
        let available_shares = 1_000;
        Ok(AvailableSharesResJs {
            available_shares_number: available_shares.to_string(),
            available_shares: available_shares.to_string(),
        })
    }

    async fn get_investor_data(&self, _: LoadInvestorParJs) -> Result<LoadInvestorResJs, FrError> {
        req_delay().await;

        Ok(LoadInvestorResJs {
            investor_shares_count: "123".to_owned(),
            investor_share: "12 %".to_owned(),

            investor_already_retrieved_amount: "11100".to_owned(),
            investor_claimable_dividend: "240".to_owned(),
            investor_claimable_dividend_microalgos: "0000".to_owned(),

            investor_locked_shares: "20".to_owned(),
            investor_unlocked_shares: "10".to_owned(),
        })
    }
}
