use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::FrError;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait InvestmentProvider {
    async fn available_shares(
        &self,
        pars: AvailableSharesParJs,
    ) -> Result<AvailableSharesResJs, FrError>;
    async fn get_investor_data(
        &self,
        pars: LoadInvestorParJs,
    ) -> Result<LoadInvestorResJs, FrError>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoadInvestorParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadInvestorResJs {
    pub investor_shares_count: String,
    pub investor_share: String, // relative to the dao's income
    pub investor_already_retrieved_amount: String,
    pub investor_claimable_dividend: String,
    pub investor_claimable_dividend_microalgos: String, // passthrough
    pub investor_locked_shares: String,
    pub investor_unlocked_shares: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvailableSharesParJs {
    pub dao_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableSharesResJs {
    pub available_shares_number: String,
    pub available_shares: String,
}
