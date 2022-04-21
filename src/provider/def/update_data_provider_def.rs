use crate::js::common::{signed_js_tx_to_signed_tx1, to_my_algo_tx1};
use crate::provider::update_data_provider::{
    SubmitUpdateDataParJs, SubmitUpdateDataResJs, UpdatableDataParJs, UpdatableDataResJs,
    UpdateDataParJs, UpdateDataProvider, UpdateDataResJs,
};
use algonaut::core::Address;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::api::version::Version;
use base::api::version::VersionedAddress;
use base::dependencies::algod;
use base::flows::create_dao::storage::load_dao::DaoId;
use base::flows::update_data::update_data::{
    submit_update_data, update_data, UpdatableDaoData, UpdateDaoDataSigned,
};
use base::funds::FundsAmount;
use base::state::dao_app_state::dao_global_state;

pub struct UpdateDataProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderDef {
    async fn updatable_data(&self, pars: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        let algod = algod();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

        let app_state = dao_global_state(&algod, dao_id.0).await?;

        Ok(UpdatableDataResJs {
            owner: app_state.owner.to_string(),
            customer_escrow: app_state.customer_escrow.address.to_string(),
            customer_escrow_version: app_state.customer_escrow.version.0.to_string(),
            project_name: app_state.project_name,
            project_desc: app_state.project_desc,
            share_price: app_state.share_price.to_string(),
            logo_url: app_state.logo_url,
            social_media_url: app_state.social_media_url,
        })
    }

    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs> {
        let algod = algod();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;
        let owner = pars.owner.parse().map_err(Error::msg)?;

        // TODO escrow versioning
        // we're currently saving only the addresses, so don't have the programs to lsig
        // so we've to store the version too (although it could be skipped by just trying all available versions against the address, which seems very inefficient)
        // and use this version to retrieve the program
        // the teal has to be updated to store the version, either in the same field as the address or a separate field with all the escrow's versions

        let to_sign = update_data(
            &algod,
            &owner,
            dao_id.0,
            &UpdatableDaoData {
                customer_escrow: VersionedAddress::new(
                    parse_addr(pars.customer_escrow)?,
                    parse_int(pars.customer_escrow_version)?,
                ),
                project_name: pars.project_name,
                project_desc: pars.project_desc,
                share_price: FundsAmount::new(pars.share_price.parse().map_err(Error::msg)?),
                logo_url: pars.logo_url,
                social_media_url: pars.social_media_url,
                owner,
            },
        )
        .await?;

        Ok(UpdateDataResJs {
            to_sign: to_my_algo_tx1(&to_sign.update).map_err(Error::msg)?,
        })
    }

    async fn submit(&self, pars: SubmitUpdateDataParJs) -> Result<SubmitUpdateDataResJs> {
        let algod = algod();

        let submit_update_res = submit_update_data(
            &algod,
            UpdateDaoDataSigned {
                update: signed_js_tx_to_signed_tx1(&pars.tx)?,
            },
        )
        .await?;

        log::debug!("Submit update dao data res: {:?}", submit_update_res);

        Ok(SubmitUpdateDataResJs {})
    }
}

fn parse_int(str: String) -> Result<Version> {
    Ok(Version(str.parse().map_err(Error::msg)?))
}

fn parse_addr(s: String) -> Result<Address> {
    s.parse().map_err(Error::msg)
}
