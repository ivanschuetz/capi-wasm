use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{error::FrError, js::bridge::log_wrap_new};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait SharesDistributionProvider {
    async fn get(&self, pars: SharedDistributionParJs) -> Result<SharedDistributionResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SharedDistributionParJs {
    pub asset_id: String,
    /// optimization to not have to fetch the asset: the asset specs are in the dao, which the frontend has to fetch first (to get the asset id)
    pub share_supply: String,

    pub app_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ShareHoldingPercentageJs {
    pub address: String,
    pub label: String,
    pub address_browser_link: String,
    pub amount: String,
    pub percentage_formatted: String,
    pub percentage_number: String,
    pub type_: String, // NOTE: don't change without updating react
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SharedDistributionResJs {
    pub holders: Vec<ShareHoldingPercentageJs>,
    pub not_owned_shares: String,
}

#[wasm_bindgen(js_name=sharesDistribution)]
pub async fn shares_distribution(
    pars: SharedDistributionParJs,
) -> Result<SharedDistributionResJs, FrError> {
    log_wrap_new("shares_distribution", pars, async move |pars| {
        providers()?.shares_distribution.get(pars).await
    })
    .await
}
