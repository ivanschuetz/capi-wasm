use std::convert::TryInto;

use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::common::to_my_algo_txs1;
use crate::model::dao_for_users::dao_to_dao_for_users;
use crate::model::dao_for_users_view_data::{dao_for_users_to_view_data, DaoForUsersViewData};
use crate::provider::create_dao_provider::{
    CreateDaoParJs, CreateDaoProvider, CreateDaoResJs, SubmitCreateDaoParJs,
    SubmitSetupDaoPassthroughParJs,
};
use crate::service::constants::PRECISION;
use algonaut::transaction::Transaction;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::api::api::Api;
use base::api::contract::Contract;
use base::dependencies::algod;
use base::flows::create_dao::model::{SetupDaoSigned, SetupDaoToSign};
use base::flows::create_dao::setup::create_shares::{submit_create_assets, CrateDaoAssetsSigned};
use base::flows::create_dao::setup_dao::{setup_dao_txs, submit_setup_dao};
use base::flows::create_dao::setup_dao::{Escrows, Programs};
use base::flows::create_dao::storage::load_dao::DaoAppId;

pub struct CreateDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateDaoProvider for CreateDaoProviderDef {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs> {
        let algod = algod();
        let api = api();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        // we assume order: js has as little logic as possible:
        // we send txs to be signed, as an array, and get the signed txs array back
        // js doesn't access the individual array txs, just passes the array to myalgo and gets signed array back
        // so this is the order in which we sent the txs to be signed, from the previously called rust fn.
        let create_shares_signed_tx = &pars.create_assets_signed_txs[0];
        let create_app_signed_tx = &pars.create_assets_signed_txs[1];

        let submit_assets_res = submit_create_assets(
            &algod,
            &CrateDaoAssetsSigned {
                create_shares: signed_js_tx_to_signed_tx1(create_shares_signed_tx)?,
                create_app: signed_js_tx_to_signed_tx1(create_app_signed_tx)?,
            },
        )
        .await?;

        let creator_address = pars.pt.inputs.creator.parse().map_err(Error::msg)?;
        let dao_specs = pars.pt.inputs.to_dao_specs(&funds_asset_specs)?;

        let last_versions = api.last_versions();

        let programs = Programs {
            central_app_approval: api
                .template(Contract::DaoAppApproval, last_versions.app_approval)?,
            central_app_clear: api.template(Contract::DaoAppClear, last_versions.app_clear)?,
            escrows: Escrows {
                customer_escrow: api
                    .template(Contract::DaoCustomer, last_versions.customer_escrow)?,
            },
        };

        let to_sign = setup_dao_txs(
            &algod,
            &dao_specs,
            creator_address,
            creator_address, // for now creator is owner
            submit_assets_res.shares_asset_id,
            funds_asset_specs.id,
            &programs,
            PRECISION,
            submit_assets_res.app_id,
            &capi_deps,
        )
        .await?;

        // double-checking total length as well, just in case
        // in the next step we also check the length of the signed txs
        let txs_to_sign = &txs_to_sign(&to_sign);
        if txs_to_sign.len() as u64 != 4 {
            return Err(anyhow!(
                "Unexpected to sign dao txs length: {}",
                txs_to_sign.len()
            ));
        }

        Ok(CreateDaoResJs {
            to_sign: to_my_algo_txs1(txs_to_sign)?,
            pt: SubmitSetupDaoPassthroughParJs {
                specs: dao_specs,
                creator: creator_address.to_string(),
                // !! TODO renamed: escrow_optin_signed_txs_msg_pack -> and only 1 tx now (not vec)
                customer_escrow_optin_to_funds_asset_tx_msg_pack: rmp_serde::to_vec_named(
                    &to_sign.customer_escrow_optin_to_funds_asset_tx,
                )?,
                shares_asset_id: submit_assets_res.shares_asset_id,
                customer_escrow: to_sign.customer_escrow.into(),
                app_id: submit_assets_res.app_id.0,
            },
        })
    }

    async fn submit(&self, pars: SubmitCreateDaoParJs) -> Result<DaoForUsersViewData> {
        // log::debug!("in bridge_submit_create_dao, pars: {:?}", pars);

        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        if pars.txs.len() != 4 {
            return Err(anyhow!(
                "Unexpected signed dao txs length: {}",
                pars.txs.len()
            ));
        }

        // TODO (low prio) improve this access, it's easy for the indices to get out of sync
        // and assign the txs to incorrect variables, which may cause subtle bugs
        // maybe refactor writing/reading into a helper struct or function
        // (written in create_dao::txs_to_sign)
        let setup_app_tx = &pars.txs[0];
        let customer_escrow_funding_tx = &pars.txs[1];
        let app_funding_tx = &pars.txs[2];
        let transfer_shares_to_app_tx = &pars.txs[3];

        log::debug!("Submitting the dao..");

        let submit_dao_res = submit_setup_dao(
            &algod,
            SetupDaoSigned {
                app_funding_tx: signed_js_tx_to_signed_tx1(app_funding_tx)?,
                fund_customer_escrow_tx: signed_js_tx_to_signed_tx1(customer_escrow_funding_tx)?,
                customer_escrow_optin_to_funds_asset_tx: rmp_serde::from_slice(
                    &pars.pt.customer_escrow_optin_to_funds_asset_tx_msg_pack,
                )
                .map_err(Error::msg)?,
                transfer_shares_to_app_tx: signed_js_tx_to_signed_tx1(transfer_shares_to_app_tx)?,
                setup_app_tx: signed_js_tx_to_signed_tx1(setup_app_tx)?,
                specs: pars.pt.specs,
                creator: pars.pt.creator.parse().map_err(Error::msg)?,
                shares_asset_id: pars.pt.shares_asset_id,
                customer_escrow: pars.pt.customer_escrow.try_into().map_err(Error::msg)?,
                funds_asset_id: funds_asset_specs.id,
                app_id: DaoAppId(pars.pt.app_id),
            },
        )
        .await?;

        log::debug!("Submit dao res: {:?}", submit_dao_res);

        Ok(dao_for_users_to_view_data(
            dao_to_dao_for_users(&submit_dao_res.dao, &submit_dao_res.dao.id())?,
            &funds_asset_specs,
        ))
    }
}

fn txs_to_sign(res: &SetupDaoToSign) -> Vec<Transaction> {
    vec![
        res.setup_app_tx.clone(),
        res.customer_escrow_funding_tx.clone(),
        res.fund_app_tx.clone(),
        res.transfer_shares_to_app_tx.clone(),
    ]
}
