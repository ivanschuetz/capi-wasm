use crate::js::explorer_links::explorer_address_link_env;
use crate::provider::shares_distribution_provider::{
    ShareHoldingPercentageJs, SharedDistributionParJs, SharedDistributionResJs,
    SharesDistributionProvider,
};
use algonaut::core::Address;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base::queries::shares_distribution::{shares_holders_distribution, ShareHoldingPercentage};
use mbase::dependencies::{algod, indexer};
use mbase::util::decimal_util::{AsDecimal, DecimalExt};

pub struct SharesDistributionProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl SharesDistributionProvider for SharesDistributionProviderDef {
    async fn get(&self, pars: SharedDistributionParJs) -> Result<SharedDistributionResJs> {
        let algod = algod();
        let indexer = indexer();

        let asset_id = pars.asset_id.parse()?;
        let share_supply = pars.share_supply.parse()?;
        let app_id = pars.app_id.parse()?;

        let holders =
            shares_holders_distribution(&algod, &indexer, asset_id, app_id, share_supply).await?;

        let mut holders_js = vec![];
        for h in &holders {
            holders_js.push(ShareHoldingPercentageJs {
                address: h.address.to_string(),
                label: shorten_address(&h.address)?,
                address_browser_link: explorer_address_link_env(&h.address),
                amount: h.amount.to_string(),
                percentage_formatted: h.percentage.format_percentage(),
                percentage_number: h.percentage.to_string(),
                type_: "holder".to_owned(),
            });
        }

        let not_owned = not_owned_shares_holdings(&holders, share_supply)?;
        let not_owned_shares = not_owned.amount.clone();
        holders_js.push(not_owned);

        Ok(SharedDistributionResJs {
            holders: holders_js,
            not_owned_shares,
        })
    }
}

// pub: shares with mock data
pub fn not_owned_shares_holdings(
    holders: &[ShareHoldingPercentage],
    supply: u64,
) -> Result<ShareHoldingPercentageJs> {
    let total_holders_amount: u64 = holders.iter().map(|h| h.amount.val()).sum();

    let not_owned_amount: u64 = supply.checked_sub(total_holders_amount).ok_or_else(|| {
        anyhow!(
            "Error supply - total amount: {} - {}",
            supply,
            total_holders_amount
        )
    })?;
    let not_owned_percentage = not_owned_amount
        .as_decimal()
        .checked_div(supply.as_decimal())
        .ok_or_else(|| {
            anyhow!("not_owned_amount: {not_owned_amount:?} / supply: {supply:?} failed")
        })?;

    Ok(ShareHoldingPercentageJs {
        address: "".to_owned(),
        label: "Not owned".to_owned(),
        address_browser_link: "".to_owned(),
        amount: not_owned_amount.to_string(),
        percentage_formatted: not_owned_percentage.format_percentage(),
        percentage_number: not_owned_percentage.to_string(),
        type_: "not_owned".to_owned(),
    })
}

// pub: shares with mock data
pub fn shorten_address(address: &Address) -> Result<String> {
    let address_str = address.to_string();

    let len = address_str.len();

    if len < 6 {
        return Err(anyhow!("Invalid address (too short): {address}"));
    }

    Ok(format!(
        "{}...{}",
        address_str[0..3].to_owned(),
        address_str[len - 3..len].to_owned()
    ))
}
