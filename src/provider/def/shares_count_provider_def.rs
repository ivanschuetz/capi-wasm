use crate::provider::shares_count_provider::{GetUserSharesCountParJs, SharesCountProvider};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::state::account_state::asset_holdings;
use mbase::dependencies::algod;

pub struct SharesCountProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl SharesCountProvider for SharesCountProviderDef {
    async fn get(&self, pars: GetUserSharesCountParJs) -> Result<String> {
        let algod = algod();

        Ok(asset_holdings(
            &algod,
            &pars.address.parse().map_err(Error::msg)?,
            pars.shares_asset_id.parse()?,
        )
        .await?
        .0
        .to_string())
    }
}
