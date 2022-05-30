use crate::{
    dependencies::{api, capi_deps, funds_asset_specs},
    js::explorer_links::explorer_tx_id_link_env,
    provider::funds_activity_provider::{
        FundsActivityProvider, FundsActivityViewData, LoadFundsActivityParJs,
        LoadFundsActivityResJs,
    },
    service::str_to_algos::base_units_to_display_units_str,
};
use anyhow::Result;
use async_trait::async_trait;
use base::{
    flows::create_dao::storage::load_dao::load_dao,
    queries::funds_activity::{funds_activity, FundsActivityEntryType},
};
use mbase::{
    checked::CheckedSub,
    dependencies::{algod, indexer},
};

use super::shares_distribution_provider_def::shorten_address;

pub struct FundsActivityProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl FundsActivityProvider for FundsActivityProviderDef {
    async fn get(&self, pars: LoadFundsActivityParJs) -> Result<LoadFundsActivityResJs> {
        let algod = algod();
        let api = api();
        let indexer = indexer();
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;
        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        let mut activity_entries = funds_activity(
            &algod,
            &indexer,
            dao_id,
            dao.customer_escrow.address(),
            &api,
            &capi_deps,
            dao.funds_asset_id,
        )
        .await?;
        // sort descendingly by date (most recent activity first)
        activity_entries.sort_by(|p1, p2| p2.date.cmp(&p1.date));

        // TODO limit results already with the queries?
        if let Some(max_results) = pars.max_results {
            let max_results = max_results.parse()?;
            activity_entries = activity_entries.into_iter().take(max_results).collect();
        }

        let mut view_data_entries = vec![];
        for entry in activity_entries {
            view_data_entries.push(FundsActivityViewData {
                amount: base_units_to_display_units_str(entry.amount, &funds_asset_specs()?),
                fee: base_units_to_display_units_str(entry.fee, &funds_asset_specs()?),
                amount_without_fee: base_units_to_display_units_str(
                    entry.amount.sub(&entry.fee)?,
                    &funds_asset_specs()?,
                ),
                is_income: match entry.type_ {
                    FundsActivityEntryType::Income => "true",
                    FundsActivityEntryType::Spending => "false",
                }
                .to_owned(),
                // TODO needs tx types (Income -> Invest, Payment)
                type_label: match entry.type_ {
                    FundsActivityEntryType::Income => "Income",
                    FundsActivityEntryType::Spending => "Withdraw",
                }
                .to_owned(),
                description: entry.description,
                date: entry.date.format("%a %b %e %Y").to_string(),
                tx_id: entry.tx_id.to_string(),
                tx_link: explorer_tx_id_link_env(&entry.tx_id),
                address: shorten_address(&entry.address)?,
            });
        }

        Ok(LoadFundsActivityResJs {
            entries: view_data_entries,
        })
    }
}
