use crate::dependencies::capi_deps;
use crate::js::common::{signed_js_tx_to_signed_tx1, to_my_algo_tx1};
use crate::provider::update_app_provider::{
    SubmitUpdateAppParJs, SubmitUpdateAppResJs, UpdateAppProvider, UpdateDaoAppParJs,
    UpdateDaoAppResJs,
};
use crate::service::constants::PRECISION;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::create_dao::setup::create_app::{
    render_and_compile_app_approval, render_and_compile_app_clear,
};
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::update_app::update::{submit_update, update, UpdateAppSigned};
use base::teal::TealApi;
use mbase::api::contract::Contract;
use mbase::api::version::Version;
use mbase::dependencies::algod;

pub struct UpdateAppProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateAppProvider for UpdateAppProviderDef {
    async fn txs(&self, pars: UpdateDaoAppParJs) -> Result<UpdateDaoAppResJs> {
        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse().map_err(Error::msg)?;
        let owner = pars.owner.parse().map_err(Error::msg)?;
        // TODO versioning
        // flow:
        // 1) user selects version number on UI (needs also a new service to check for and display new versions)
        // 2) fetch template for that version number (e.g. using strings like currently or download from somewhere)
        // 3) call redering function for that version (should be implemented in core)
        // Note that the current core "render_central_app" function is essentially for version 1.
        // Side note: consider adding version as a comment in TEAL and check in the render functions (for a bit more security re: passing the correct template versions to the rendering functions)
        let approval_version: Version = Version(pars.approval_version.parse().map_err(Error::msg)?);
        let approval_template = api
            .template(Contract::DaoAppApproval, approval_version)
            .await?;

        let clear_version: Version = Version(pars.approval_version.parse().map_err(Error::msg)?);
        let clear_template = api.template(Contract::DaoAppClear, clear_version).await?;

        // TODO optimize: instead of calling load_dao, fetch app state and asset infos (don't e.g. compile and render the escrows, which is not needed here)
        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        // TODO versioning
        // since there's no versioning, we just render again with v1
        let app_source = render_and_compile_app_approval(
            &algod,
            &approval_template,
            dao.specs.shares.supply,
            PRECISION,
            dao.specs.investors_share,
            &capi_deps.address,
            capi_deps.escrow_percentage,
            dao.specs.share_price,
        )
        .await?;
        let clear_source = render_and_compile_app_clear(&algod, &clear_template).await?;

        let to_sign = update(&algod, &owner, dao_id.0, app_source, clear_source).await?;

        Ok(UpdateDaoAppResJs {
            to_sign: to_my_algo_tx1(&to_sign.update).map_err(Error::msg)?,
        })
    }

    async fn submit(&self, pars: SubmitUpdateAppParJs) -> Result<SubmitUpdateAppResJs> {
        let algod = algod();

        let submit_update_res = submit_update(
            &algod,
            UpdateAppSigned {
                update: signed_js_tx_to_signed_tx1(&pars.tx)?,
            },
        )
        .await?;

        log::debug!("Submit update res: {:?}", submit_update_res);

        Ok(SubmitUpdateAppResJs {})
    }
}
