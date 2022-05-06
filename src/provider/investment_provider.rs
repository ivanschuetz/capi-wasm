use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::dependencies::FundsAssetSpecs;

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
    pub available_shares: String,                       // shares that can be purchased in the Dao
    pub investor_locked_shares: String,
    pub investor_unlocked_shares: String,
    // basically the share price
    // naming is UI oriented: it's the price we show at the beginning, before the user makes any inputs,
    // which would update the shown price to the entered amount * price
    pub init_share_price: String,
    // percentage of profit corresponding to init share price,
    // we act as if the user entered 1 share (price corresponds to 1 share) and show the profit % for that
    // also updated when the user enters an amount
    pub init_profit_percentage: String,
    pub share_specs_msg_pack: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcPriceAndPercSpecs {
    pub funds_specs: FundsAssetSpecs,
    // pub funds_specs: FundsAssetSpecs,
}
