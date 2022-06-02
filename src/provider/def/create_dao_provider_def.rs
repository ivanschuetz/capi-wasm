use std::convert::TryInto;

use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::common::to_my_algo_txs1;
use crate::model::dao_js::ToDaoJs;
use crate::provider::create_dao_provider::{
    CreateDaoParJs, CreateDaoProvider, CreateDaoRes, CreateDaoResJs, SubmitCreateDaoParJs,
    SubmitSetupDaoPassthroughParJs,
};
use crate::service::constants::PRECISION;
use algonaut::algod::v2::Algod;
use algonaut::transaction::Transaction;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::api::image_api::ImageApi;
use base::dependencies::{image_api, teal_api};
use base::flows::create_dao::model::{SetupDaoSigned, SetupDaoToSign};
use base::flows::create_dao::setup::create_shares::{submit_create_assets, CrateDaoAssetsSigned};
use base::flows::create_dao::setup_dao::{setup_dao_txs, submit_setup_dao};
use base::flows::create_dao::setup_dao::{Escrows, Programs};
use base::flows::create_dao::setup_dao_specs::{CompressedImage, HashableString};
use base::flows::create_dao::storage::load_dao::TxId;
use base::network_util::wait_for_pending_transaction;
use base::teal::TealApi;
use mbase::api::contract::Contract;
use mbase::dependencies::algod;
use mbase::models::dao_app_id::DaoAppId;
use mbase::models::hash::GlobalStateHash;

pub struct CreateDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateDaoProvider for CreateDaoProviderDef {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs> {
        let algod = algod();
        let api = teal_api();
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

        let last_versions = api.last_versions().await?;

        let programs = Programs {
            central_app_approval: api
                .template(Contract::DaoAppApproval, last_versions.app_approval)
                .await?,
            central_app_clear: api
                .template(Contract::DaoAppClear, last_versions.app_clear)
                .await?,
            escrows: Escrows {
                customer_escrow: api
                    .template(Contract::DaoCustomer, last_versions.customer_escrow)
                    .await?,
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
                description: pars.pt.inputs.dao_description,
                compressed_image: pars.pt.inputs.compressed_image,
            },
        })
    }

    async fn submit(&self, pars: SubmitCreateDaoParJs) -> Result<CreateDaoRes> {
        // log::debug!("in bridge_submit_create_dao, pars: {:?}", pars);

        let algod = algod();
        let image_api = image_api();
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

        // clone descr_hash here to be able to use it after passing specs to signed struct
        let descr_hash = pars.pt.specs.descr_hash.clone();
        // clone image_hash here to be able to use it after passing specs to signed struct
        let image_hash = pars.pt.specs.image_hash.clone();

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

        // Note that it's required to upload the description after DAO setup, because the api checks the hash in the app's global state.
        let (_, maybe_descr_upload_error) = maybe_upload_descr(
            &algod,
            &image_api,
            submit_dao_res.tx_id.clone(),
            DaoAppId(pars.pt.app_id),
            pars.pt.description,
            descr_hash.clone(),
        )
        .await?;

        // Note that it's required to upload the image after DAO setup, because the api checks the hash in the app's global state.
        let (maybe_image_url, maybe_image_upload_error) = maybe_upload_image(
            &algod,
            &image_api,
            submit_dao_res.tx_id,
            DaoAppId(pars.pt.app_id),
            pars.pt.compressed_image.map(CompressedImage::new),
            image_hash,
        )
        .await?;

        Ok(CreateDaoRes {
            dao: submit_dao_res.dao.to_js(
                descr_hash.map(|h| h.as_str()),
                maybe_image_url,
                &funds_asset_specs,
            ),
            descr_error: maybe_descr_upload_error,
            image_error: maybe_image_upload_error,
        })
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

/// Returns: Url of the uploaded image (if upload was succesful), error message otherwise
/// The error message is not an error as we don't want to abort the DAO setup (which with current implementation would leave the user in an incomplete state),
/// we only show a message to the user and tell them to try again later from the settings
/// this flow may be improved in the future
pub async fn maybe_upload_image(
    algod: &Algod,
    api: &dyn ImageApi,
    tx_id_to_wait: TxId,
    app_id: DaoAppId,
    image: Option<CompressedImage>,
    image_hash: Option<GlobalStateHash>,
) -> Result<(Option<String>, Option<String>)> {
    // Note that it's required to upload the image after DAO setup, because the image api checks that the hash is in the app's global state.
    match (image, image_hash) {
        (Some(image), Some(hash)) => {
            // double checking that the hash which we stored in the DAO (passed to the setup dao tx when generating the txs)
            // matches the just calculated hash of the image (which we get from passthrough data)
            // no specific reason for why they should be different, but better more checks than less
            if image.hash() != hash {
                return Err(anyhow!("Passthrough image hash != image hash"));
            }
            upload_image(algod, api, tx_id_to_wait, app_id, hash, image).await
        }
        // user provided no image: no image url, no error
        (None, None) => Ok((None, None)),
        _ => Err(anyhow!(
            "Invalid combination: either image and hash are set or none are set"
        )),
    }
}

/// Returns: Url of the uploaded image (if upload was succesful), error message otherwise
/// The error message is not an error as we don't want to abort the DAO setup (which with current implementation would leave the user in an incomplete state),
/// we only show a message to the user and tell them to try again later from the settings
/// this flow may be improved in the future
async fn upload_image(
    algod: &Algod,
    api: &dyn ImageApi,
    tx_id_to_wait: TxId,
    app_id: DaoAppId,
    image_hash: GlobalStateHash,
    image: CompressedImage,
) -> Result<(Option<String>, Option<String>)> {
    wait_for_pending_transaction(algod, &tx_id_to_wait).await?;
    let (possible_image_url, possible_image_upload_error) = match api
            .upload_image(app_id, image.bytes())
            .await {
                Ok(_) => (Some(image_hash.as_api_id()), None),
                Err(e) => (None, Some(format!("Error storing image: {e}. Please try uploading it again from the project's settings.")))
            };

    Ok((possible_image_url, possible_image_upload_error))
}

/// Returns: Url of the uploaded descr (if upload was succesful), error message otherwise
/// The error message is not an error as we don't want to abort the DAO setup (which with current implementation would leave the user in an incomplete state),
/// we only show a message to the user and tell them to try again later from the settings
/// this flow may be improved in the future
/// TODO refactor with maybe_upload_image
pub async fn maybe_upload_descr(
    algod: &Algod,
    api: &dyn ImageApi,
    tx_id_to_wait: TxId,
    app_id: DaoAppId,
    descr: Option<String>,
    descr_hash: Option<GlobalStateHash>,
) -> Result<(Option<String>, Option<String>)> {
    // Note that it's required to upload the image after DAO setup, because the image api checks that the hash is in the app's global state.
    match (descr, descr_hash) {
        (Some(descr), Some(hash)) => {
            // double checking that the hash which we stored in the DAO (passed to the setup dao tx when generating the txs)
            // matches the just calculated hash of the image (which we get from passthrough data)
            // no specific reason for why they should be different, but better more checks than less
            if descr.hash() != hash {
                return Err(anyhow!("Passthrough descr hash != descr hash"));
            }
            upload_descr(algod, api, tx_id_to_wait, app_id, hash, descr).await
        }
        // user provided no image: no image url, no error
        (None, None) => Ok((None, None)),
        _ => Err(anyhow!(
            "Invalid combination: either descr and hash are set or none are set"
        )),
    }
}

/// Returns: Url of the uploaded descr (if upload was succesful), error message otherwise
/// The error message is not an error as we don't want to abort the DAO setup (which with current implementation would leave the user in an incomplete state),
/// we only show a message to the user and tell them to try again later from the settings
/// this flow may be improved in the future
/// TODO refactor with upload_image
async fn upload_descr(
    algod: &Algod,
    api: &dyn ImageApi,
    tx_id_to_wait: TxId,
    app_id: DaoAppId,
    descr_hash: GlobalStateHash,
    descr: String,
) -> Result<(Option<String>, Option<String>)> {
    wait_for_pending_transaction(algod, &tx_id_to_wait).await?;
    let (possible_image_url, possible_image_upload_error) = match api
            .upload_descr(app_id, descr.as_bytes().to_vec())
            .await {
                Ok(_) => (Some(descr_hash.as_api_id()), None),
                Err(e) => (None, Some(format!("Error storing image: {e}. Please try uploading it again from the project's settings.")))
            };

    Ok((possible_image_url, possible_image_upload_error))
}
