use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait SharesDistributionProvider {
    async fn get(&self, pars: SharedDistributionParJs) -> Result<SharedDistributionResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct SharedDistributionParJs {
    pub asset_id: String,
    /// optimization to not have to fetch the asset: the asset specs are in the dao, which the frontend has to fetch first (to get the asset id)
    pub share_supply: String,

    pub app_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShareHoldingPercentageJs {
    pub address: String,
    pub label: String,
    pub address_browser_link: String,
    pub amount: String,
    pub percentage_formatted: String,
    pub percentage_number: String,
    pub type_: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SharedDistributionResJs {
    pub holders: Vec<ShareHoldingPercentageJs>,
}
