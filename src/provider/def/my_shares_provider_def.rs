use crate::dependencies::capi_deps;
use crate::provider::my_shares_provider::{MySharesParJs, MySharesProvider, MySharesResJs};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::state::account_state::asset_holdings;
use mbase::checked::CheckedAdd;
use mbase::dependencies::algod;
use mbase::models::share_amount::ShareAmount;
use mbase::state::app_state::ApplicationLocalStateError;
use mbase::state::dao_app_state::dao_investor_state;

pub struct MySharesProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl MySharesProvider for MySharesProviderDef {
    async fn get(&self, pars: MySharesParJs) -> Result<MySharesResJs> {
        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        log::debug!("Dao: {dao:?}");

        let my_address = &pars.my_address.parse().map_err(Error::msg)?;

        let locked_shares = match dao_investor_state(&algod, my_address, dao.app_id).await {
            Ok(state) => state.shares,
            Err(ApplicationLocalStateError::NotOptedIn) => ShareAmount::new(0), // not invested -> 0 shares
            Err(e) => return Err(Error::msg(e)),
        };

        let free_shares = match asset_holdings(&algod, my_address, dao.shares_asset_id).await {
            Ok(shares) => ShareAmount(shares),
            Err(e) => return Err(Error::msg(e)),
        };

        let total_shares = locked_shares.add(&free_shares)
            .map_err(|e| anyhow!("Invalid state: locked shares: {locked_shares} + fee_shares: {free_shares} caused an overflow. This is expected to be <= asset supply, which is an u64. e: {e:?}"))?;

        Ok(MySharesResJs {
            total: total_shares.0.to_string(),
            free: free_shares.to_string(),
            locked: locked_shares.to_string(),
        })
    }
}
