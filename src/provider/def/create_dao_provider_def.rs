use crate::dependencies::funds_asset_specs;
use crate::error::FrError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::model::dao_js::ToDaoJs;
use crate::provider::create_dao_provider::{
    validate_dao_inputs, validated_inputs_to_dao_specs, CreateDaoParJs, CreateDaoProvider,
    CreateDaoRes, CreateDaoResJs, SubmitCreateDaoParJs, SubmitSetupDaoPassthroughParJs,
};
use crate::service::constants::PRECISION;
use algonaut::transaction::Transaction;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::create_dao::model::{SetupDaoSigned, SetupDaoToSign};
use base::flows::create_dao::setup::create_shares::{submit_create_assets, CreateDaoAssetsSigned};
use base::flows::create_dao::setup_dao::Programs;
use base::flows::create_dao::setup_dao::{setup_dao_txs, submit_setup_dao};
use base::teal::TealApi;
use mbase::api::contract::Contract;
use mbase::dependencies::algod;
use mbase::models::dao_app_id::DaoAppId;
use mbase::models::timestamp::Timestamp;

pub struct CreateDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateDaoProvider for CreateDaoProviderDef {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs, FrError> {
        let algod = algod();
        let api = teal_api();
        let funds_asset_specs = funds_asset_specs()?;

        // we assume order: js has as little logic as possible:
        // we send txs to be signed, as an array, and get the signed txs array back
        // js doesn't access the individual array txs, just passes the array to myalgo and gets signed array back
        // so this is the order in which we sent the txs to be signed, from the previously called rust fn.
        let create_shares_signed_tx = &pars.create_assets_signed_txs[0];
        let create_app_signed_tx = &pars.create_assets_signed_txs[1];

        let validated_inputs = validate_dao_inputs(&pars.pt.inputs, &funds_asset_specs)?;

        let submit_assets_res = submit_create_assets(
            &algod,
            &CreateDaoAssetsSigned {
                create_shares: signed_js_tx_to_signed_tx1(create_shares_signed_tx)?,
                create_app: signed_js_tx_to_signed_tx1(create_app_signed_tx)?,
            },
        )
        .await;

        if let Some(err) = submit_assets_res.as_ref().err() {
            if err.to_string().contains("overspend") {
                return Err(FrError::NotEnoughAlgos);
            }
        }

        let submit_assets_res = submit_assets_res?;

        let creator_address = pars.pt.inputs.creator.parse().map_err(Error::msg)?;
        let dao_specs = validated_inputs_to_dao_specs(&validated_inputs)?;

        let last_versions = api.last_versions().await?;

        let programs = Programs {
            central_app_approval: api
                .template(Contract::DaoAppApproval, last_versions.app_approval)
                .await?,
            central_app_clear: api
                .template(Contract::DaoAppClear, last_versions.app_clear)
                .await?,
        };

        let to_sign = setup_dao_txs(
            &algod,
            &dao_specs,
            creator_address,
            submit_assets_res.shares_asset_id,
            funds_asset_specs.id,
            &programs,
            PRECISION,
            submit_assets_res.app_id,
            dao_specs.image_url.clone(),
            dao_specs.prospectus_url.clone(),
        )
        .await?;

        // double-checking total length as well, just in case
        // in the next step we also check the length of the signed txs
        let txs_to_sign = txs_to_sign(&to_sign);
        if txs_to_sign.len() as u64 != 3 {
            return Err(FrError::Msg(format!(
                "Unexpected to sign dao txs length: {}",
                txs_to_sign.len()
            )));
        }

        Ok(CreateDaoResJs {
            to_sign: ToSignJs::new(txs_to_sign)?,
            pt: SubmitSetupDaoPassthroughParJs {
                specs: dao_specs,
                creator: creator_address.to_string(),
                shares_asset_id: submit_assets_res.shares_asset_id,
                app_id: submit_assets_res.app_id.0,
                description_url: validated_inputs.description_url,
                setup_date: to_sign.setup_date.0.to_string(),
            },
        })
    }

    async fn submit(&self, pars: SubmitCreateDaoParJs) -> Result<CreateDaoRes> {
        // log::debug!("in bridge_submit_create_dao, pars: {:?}", pars);

        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        if pars.txs.len() != 3 {
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
        let app_funding_tx = &pars.txs[1];
        let transfer_shares_to_app_tx = &pars.txs[2];

        log::debug!("Submitting the dao..");

        let setup_date: Timestamp = Timestamp(pars.pt.setup_date.parse()?);

        let submit_dao_res = submit_setup_dao(
            &algod,
            SetupDaoSigned {
                app_funding_tx: signed_js_tx_to_signed_tx1(app_funding_tx)?,
                transfer_shares_to_app_tx: signed_js_tx_to_signed_tx1(transfer_shares_to_app_tx)?,
                setup_app_tx: signed_js_tx_to_signed_tx1(setup_app_tx)?,
                specs: pars.pt.specs.clone(),
                creator: pars.pt.creator.parse().map_err(Error::msg)?,
                shares_asset_id: pars.pt.shares_asset_id,
                funds_asset_id: funds_asset_specs.id,
                app_id: DaoAppId(pars.pt.app_id),
                image_url: pars.pt.specs.image_url,
                setup_date,
            },
        )
        .await?;

        log::debug!("Submit dao res: {:?}", submit_dao_res);

        Ok(CreateDaoRes {
            dao: submit_dao_res.dao.to_js(&funds_asset_specs)?,
        })
    }
}

fn txs_to_sign(res: &SetupDaoToSign) -> Vec<Transaction> {
    vec![
        res.setup_app_tx.clone(),
        res.fund_app_tx.clone(),
        res.transfer_shares_to_app_tx.clone(),
    ]
}
