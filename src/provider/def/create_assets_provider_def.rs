use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::to_my_algo_txs1;
use crate::provider::create_assets_provider::{
    CreateAssetsProvider, CreateDaoAssetsParJs, CreateDaoAssetsResJs,
};
use crate::provider::create_dao_provider::validate_dao_inputs;
use crate::provider::create_dao_provider::{CreateDaoFormInputsJs, CreateDaoPassthroughParJs};
use crate::service::constants::PRECISION;
use algonaut::core::Address;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::api::teal_api::TealApi;
use base::api::contract::Contract;
use base::dependencies::algod;
use base::flows::create_dao::setup::create_shares::create_assets;
use base::flows::create_dao::setup_dao_specs::SetupDaoSpecs;

pub struct CreateAssetsProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateAssetsProvider for CreateAssetsProviderDef {
    async fn txs(&self, pars: CreateDaoAssetsParJs) -> Result<CreateDaoAssetsResJs> {
        let funds_asset_specs = funds_asset_specs()?;

        // Note: partly redundant validation here (to_dao_specs validates everything again)
        let validated_inputs = validate_dao_inputs(&pars.inputs, &funds_asset_specs)?;
        let dao_specs = pars.inputs.to_dao_specs(&funds_asset_specs)?;

        create_dao_assets_txs(&dao_specs, &validated_inputs.creator, pars.inputs).await
    }
}

async fn create_dao_assets_txs(
    dao_specs: &SetupDaoSpecs,
    creator: &Address,
    inputs: CreateDaoFormInputsJs,
) -> Result<CreateDaoAssetsResJs> {
    let algod = algod();
    let api = api();
    let capi_deps = capi_deps()?;

    let last_versions = api.last_versions();
    let last_approval_tmpl = api.template(Contract::DaoAppApproval, last_versions.app_approval)?;
    let last_clear_tmpl = api.template(Contract::DaoAppClear, last_versions.app_clear)?;

    let create_assets_txs = create_assets(
        &algod,
        creator,
        dao_specs,
        &last_approval_tmpl,
        &last_clear_tmpl,
        PRECISION,
        &capi_deps,
    )
    .await?;

    Ok(CreateDaoAssetsResJs {
        to_sign: to_my_algo_txs1(&[
            create_assets_txs.create_shares_tx,
            create_assets_txs.create_app_tx,
        ])
        .map_err(Error::msg)?,
        // we forward the inputs to the next step, just for a little convenience (javascript could pass them as separate fields again instead)
        // the next step will validate them again, as this performs type conversion too (+ general safety)
        pt: CreateDaoPassthroughParJs { inputs },
    })
}
