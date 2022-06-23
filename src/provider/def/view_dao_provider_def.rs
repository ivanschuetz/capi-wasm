use crate::dependencies::funds_asset_specs;
use crate::model::dao_js::ToDaoJs;
use crate::provider::view_dao_provider::{ViewDaoParJs, ViewDaoProvider, ViewDaoResJs};
use crate::service::available_funds::available_funds;
use crate::service::number_formats::base_units_to_display_units;
use crate::GlobalStateHashExt;
use algonaut::core::MicroAlgos;
use algonaut::transaction::url::LinkableTransactionBuilder;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base::dependencies::image_api;
use base::flows::create_dao::storage::load_dao::load_dao;
use mbase::dependencies::algod;
use mbase::util::decimal_util::DecimalExt;

pub struct ViewDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ViewDaoProvider for ViewDaoProviderDef {
    async fn get(&self, pars: ViewDaoParJs) -> Result<ViewDaoResJs> {
        let algod = algod();
        let image_api = image_api();
        let funds_asset_specs = funds_asset_specs()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id).await?;

        // TODO investor count: get all holders of asset (indexer?)

        let customer_payment_deeplink =
            LinkableTransactionBuilder::payment(dao.app_address(), MicroAlgos(0))
                .build()
                .as_url();

        let available_funds = available_funds(&algod, &dao, funds_asset_specs.id).await?;

        // TODO!! not-locked shares (use global function to get not-locked (name prob. "available" shares))
        let shares_available = algod
            .account_information(&dao.app_address())
            .await?
            .assets
            .iter()
            .find(|a| a.asset_id == dao.shares_asset_id)
            .ok_or({
                anyhow!("Invalid app state: Investor escrow doesn't have shares asset, Please contact support.")
            })?.amount;

        let investos_share_formatted = dao.investors_share.value().format_percentage();

        let dao_view_data = dao.to_js(
            dao.descr_hash.clone().map(|h| h.as_str()),
            dao.image_hash.clone().map(|h| h.as_api_url(&image_api)),
            &funds_asset_specs,
        )?;

        Ok(ViewDaoResJs {
            dao: dao_view_data,
            shares_available: shares_available.to_string(),
            investors_share: investos_share_formatted,
            available_funds: base_units_to_display_units(available_funds, &funds_asset_specs)
                .to_string(),
            customer_payment_deeplink: customer_payment_deeplink.to_string(),
        })
    }
}
