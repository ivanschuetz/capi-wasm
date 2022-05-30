use std::cmp::Ordering;

use crate::dependencies::{api, capi_deps};
use crate::provider::balance_provider::{
    BalanceChangeParJs, BalanceChangeResJs, BalanceParJs, BalanceProvider, BalanceResJs,
};
use crate::service::str_to_algos::microalgos_to_algos_str;
use crate::{
    dependencies::funds_asset_specs, service::str_to_algos::base_units_to_display_units_str,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::queries::historic_balance::historic_dao_funds_balance;
use base::state::account_state::{funds_holdings, funds_holdings_from_account};
use chrono::{Duration, Utc};
use mbase::dependencies::{algod, indexer};
use mbase::models::dao_id::DaoId;

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

    // TODO move somewhere else thsi is for dao funds not user balance
    async fn get_balance_change(&self, pars: BalanceChangeParJs) -> Result<BalanceChangeResJs> {
        let algod = algod();
        let indexer = indexer();
        let api = api();
        let capi_deps = capi_deps()?;
        let funds_asset_specs = funds_asset_specs()?;

        // let address = pars.address.parse().map_err(Error::msg)?;
        let dao_id: DaoId = pars.dao_id.parse()?;
        let customer_escrow_address = pars.customer_escrow.parse().map_err(Error::msg)?;

        let dao_address = dao_id.0.address();

        let date = Utc::now() - Duration::weeks(1); // account's balance a week ago
                                                    // let date = Utc::now(); // debugging: use this to get current balance

        let past_balance = historic_dao_funds_balance(
            &algod,
            &indexer,
            &api,
            funds_asset_specs.id,
            &customer_escrow_address,
            dao_id,
            &capi_deps,
            date,
        )
        .await?;
        let current_balance = funds_holdings(&algod, &dao_address, funds_asset_specs.id).await?;
        log::debug!("past balance: {past_balance:?}");
        log::debug!("current balance: {current_balance:?}");

        let change_str = match current_balance.val().cmp(&past_balance.val()) {
            Ordering::Less => "down",
            Ordering::Equal => "eq",
            Ordering::Greater => "up",
        };

        Ok(BalanceChangeResJs {
            change: change_str.to_owned(),
        })
    }
}
