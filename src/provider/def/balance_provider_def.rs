use crate::provider::balance_provider::{BalanceParJs, BalanceProvider, BalanceResJs};
use crate::service::str_to_algos::microalgos_to_algos_str;
use crate::{
    dependencies::funds_asset_specs, service::str_to_algos::base_units_to_display_units_str,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::algod;
use base::state::account_state::funds_holdings_from_account;

pub struct BalanceProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BalanceProvider for BalanceProviderDef {
    async fn get(&self, pars: BalanceParJs) -> Result<BalanceResJs> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let account = algod
            .account_information(&pars.address.parse().map_err(Error::msg)?)
            .await?;

        let balance = account.amount;

        let funds_asset_holdings = funds_holdings_from_account(&account, funds_asset_specs.id)?;

        Ok(BalanceResJs {
            balance_algos: microalgos_to_algos_str(balance),
            balance_funds_asset: base_units_to_display_units_str(
                funds_asset_holdings,
                &funds_asset_specs,
            ),
        })
    }
}
