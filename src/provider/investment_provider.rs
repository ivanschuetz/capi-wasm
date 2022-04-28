use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait InvestmentProvider {
    async fn get(&self, pars: LoadInvestmentParJs) -> Result<LoadInvestmentResJs>;
}

// TODO rename structs in BuyShares*
#[derive(Debug, Clone, Deserialize)]
pub struct LoadInvestmentParJs {
    pub dao_id: String,
    pub investor_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoadInvestmentResJs {
    pub investor_shares_count: String,
    pub investor_percentage: String,
    pub investor_percentage_number: String, // relative to investor's share (part reserved to investors)
    pub investor_percentage_relative_to_total_number: String, // relative to all the dao's income
    pub investor_already_retrieved_amount: String,
    pub investor_claimable_dividend: String,
    pub investor_claimable_dividend_microalgos: String, // passthrough
}
