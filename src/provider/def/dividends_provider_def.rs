use super::investment_provider_def::{fetch_claimable_dividend, investor_local_state_view_data};
use crate::dependencies::{api, capi_deps};
use crate::provider::dividends_provider::{DividendsParJs, DividendsProvider};
use crate::service::constants::PRECISION;
use crate::{
    dependencies::funds_asset_specs, service::number_formats::base_units_to_display_units_str,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::load_dao;
use mbase::dependencies::algod;
use mbase::models::dao_id::DaoId;
use mbase::state::dao_app_state::dao_global_state;

pub struct DividendsProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DividendsProvider for DividendsProviderDef {
    async fn get(&self, pars: DividendsParJs) -> Result<String> {
        let algod = algod();
        let api = api();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;
        let dao_id: DaoId = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;
        let central_state = dao_global_state(&algod, dao_id.0).await?;

        let investor_view_data =
            investor_local_state_view_data(&algod, investor_address, dao.app_id).await?;

        let claimable_dividend = fetch_claimable_dividend(
            &algod,
            investor_view_data.claimed,
            investor_view_data.locked_shares,
            PRECISION,
            &dao,
            &capi_deps,
            &funds_asset_specs,
            &central_state,
        )
        .await?;

        Ok(base_units_to_display_units_str(
            claimable_dividend,
            &funds_asset_specs,
        ))
    }
}
