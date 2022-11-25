use crate::error::FrError;
use crate::inputs_validation::ValidationError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::create_dao_provider::{
    validate_dao_description_url_opt, validate_dao_name, validate_image_url,
    validate_max_invest_amount, validate_min_invest_amount, validate_prospectus_bytes,
    validate_prospectus_url, validate_social_media_url, validate_text_min_max_length,
};
use crate::provider::update_data_provider::{
    SubmitUpdateDataParJs, UpdatableDataParJs, UpdatableDataResJs, UpdateDataParJs,
    UpdateDataPassthroughJs, UpdateDataProvider, UpdateDataResJs,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::api::fetcher::Fetcher;
use base::dependencies::fetcher;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::update_data::update_data::{
    submit_update_data, update_data, UpdatableDaoData, UpdateDaoDataSigned,
};
use data_encoding::BASE64;
use mbase::dependencies::algod;
use mbase::models::dao_id::DaoId;
use mbase::state::dao_app_state::{dao_global_state, Prospectus};
use serde::Serialize;
use std::collections::HashMap;

pub struct UpdateDataProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UpdateDataProvider for UpdateDataProviderDef {
    async fn get(&self, pars: UpdatableDataParJs) -> Result<UpdatableDataResJs> {
        let algod = algod();
        let fetcher = fetcher();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;

        let app_state = dao_global_state(&algod, dao_id.0).await?;

        let dao = load_dao(&algod, dao_id).await?;

        // TODO optimize: fetch description separately, DaoJs has just url
        let description = match dao.descr_url {
            Some(descr) => Some(String::from_utf8(fetcher.get(&descr).await?)?),
            None => None,
        };

        let image_base64 = match dao.image_nft {
            Some(nft) => {
                let bytes = fetcher.get(&nft.url).await?;
                let base64 = BASE64.encode(&bytes);
                Some(base64)
            }
            None => None,
        };

        Ok(UpdatableDataResJs {
            project_name: app_state.project_name,
            project_desc: description,
            share_price: app_state.share_price.to_string(),

            image_base64,
            social_media_url: app_state.social_media_url,

            prospectus: app_state.prospectus.clone(),
            min_invest_amount: app_state.min_invest_amount.val().to_string(),
            max_invest_amount: app_state.max_invest_amount.val().to_string(),
        })
    }

    async fn txs(&self, pars: UpdateDataParJs) -> Result<UpdateDataResJs, FrError> {
        let algod = algod();

        let dao_id = pars.dao_id.parse::<DaoId>().map_err(Error::msg)?;
        let owner = pars.owner.parse().map_err(Error::msg)?;

        // TODO escrow versioning
        // we're currently saving only the addresses, so don't have the programs to lsig
        // so we've to store the version too (although it could be skipped by just trying all available versions against the address, which seems very inefficient)
        // and use this version to retrieve the program
        // the teal has to be updated to store the version, either in the same field as the address or a separate field with all the escrow's versions

        let updatable_data = validate_inputs(pars)?;
        let to_sign = update_data(&algod, &owner, dao_id.0, &updatable_data).await?;

        let mut txs = vec![to_sign.update];
        if let Some(pay) = to_sign.increase_min_balance_tx {
            txs.push(pay);
        }

        Ok(UpdateDataResJs {
            to_sign: ToSignJs::new(txs)?,
            pt: UpdateDataPassthroughJs {
                dao_id: dao_id.to_string(),
            },
        })
    }

    async fn submit(&self, pars: SubmitUpdateDataParJs) -> Result<()> {
        let algod = algod();

        if pars.txs.len() != 1 && pars.txs.len() != 2 {
            return Err(anyhow!(
                "Unexpected update app data txs length: {}",
                pars.txs.len()
            ));
        }
        let update_tx = &pars.txs[0];
        let maybe_increase_min_balance_tx = if pars.txs.len() == 2 {
            Some(pars.txs[1].clone())
        } else {
            None
        };

        submit_update_data(
            &algod,
            UpdateDaoDataSigned {
                update: signed_js_tx_to_signed_tx1(update_tx)?,
                increase_min_balance_tx: match maybe_increase_min_balance_tx {
                    Some(tx) => Some(signed_js_tx_to_signed_tx1(&tx)?),
                    None => None,
                },
            },
        )
        .await?;

        Ok(())
    }
}

/// validates and returns valid data to submit update if successful
/// it returns additionally the image, which is returned to js
fn validate_inputs(
    pars: UpdateDataParJs,
) -> Result<UpdatableDaoData, ValidateDataUpdateInputsError> {
    let dao_name_res = validate_dao_name(&pars.project_name);
    let dao_description_res = validate_dao_description_url_opt(&pars.project_desc_url);
    let image_url_res = validate_image_url(&pars.image_url);
    let social_media_url_res = validate_social_media_url(&pars.social_media_url);
    let prospectus_url_res = validate_prospectus_url(&pars.prospectus_url);
    let prospectus_bytes_res = validate_prospectus_bytes(&pars.prospectus_bytes);
    let prospectus_hash_res = validate_prospectus_hash(&pars.prospectus_hash);
    let min_invest_shares_res = validate_min_invest_amount(&pars.min_invest_amount);
    let max_invest_shares_res = validate_max_invest_amount(&pars.max_invest_amount);

    match (
        &dao_name_res,
        &dao_description_res,
        &image_url_res,
        &social_media_url_res,
        &prospectus_url_res,
        &prospectus_bytes_res,
        &prospectus_hash_res,
        &min_invest_shares_res,
        &max_invest_shares_res,
    ) {
        (
            Ok(dao_name),
            Ok(dao_descr),
            Ok(image_url),
            Ok(social_media_url),
            Ok(prospectus_url),
            Ok(prospectus_bytes),
            Ok(prospectus_hash),
            Ok(min_invest_shares),
            Ok(max_invest_shares),
        ) => {
            if prospectus_bytes.is_some() && prospectus_hash.is_some() {
                // there are OR, so can't be set at the same time
                return Err(ValidateDataUpdateInputsError::NonValidation(
                    "prospectus_bytes_and_hash_set".to_owned(),
                ));
            }

            let prospectus = to_maybe_prospectus(
                prospectus_url.clone(),
                prospectus_hash.clone(),
                prospectus_bytes.clone(),
            )?;

            Ok(UpdatableDaoData {
                project_name: dao_name.clone(),
                project_desc_url: dao_descr.clone(),
                image_url: image_url.clone(),
                social_media_url: social_media_url.clone(),
                prospectus,
                min_invest_shares: min_invest_shares.clone(),
                max_invest_shares: max_invest_shares.clone(),
            })
        }
        _ => Err(ValidateDataUpdateInputsError::AllFieldsValidation(
            ValidateUpateDataInputErrors {
                name: dao_name_res.err(),
                description: dao_description_res.err(),
                image_url: image_url_res.err(),
                social_media_url: social_media_url_res.err(),
                prospectus_url: prospectus_url_res.err(),
                prospectus_bytes: prospectus_bytes_res.err(),
                prospectus_hash: prospectus_hash_res.err(),
                min_invest_shares: min_invest_shares_res.err(),
                max_invest_shares: max_invest_shares_res.err(),
            },
        )),
    }
}

/// If hash is set, it means that we're getting an unchanged prospectus (the hash of the already saved prospectus)
/// If bytes is set, it means that we're getting a new prospectus (the hash will be generated)
/// Hash and bytes must not be set at the same time. Url must always be set.
fn to_maybe_prospectus(
    url: Option<String>,
    hash: Option<String>,
    bytes: Option<Vec<u8>>,
) -> Result<Option<Prospectus>, ValidateDataUpdateInputsError> {
    let existing_prospectus = match (hash, url.clone()) {
        (Some(hash), Some(url)) => Some(Prospectus { hash, url }),
        (None, None) => None,
        (None, Some(_)) => None,
        _ => Err(ValidateDataUpdateInputsError::NonValidation(
            "Invalid combination: if there's a prospectus hash, there must be an URL too"
                .to_owned(),
        ))?,
    };

    Ok(if existing_prospectus.is_some() {
        existing_prospectus
    } else {
        match (bytes, url) {
            (Some(bytes), Some(url)) => Some(Prospectus::new(&bytes, url)),
            (None, None) => None,
            _ => Err(ValidateDataUpdateInputsError::NonValidation(
                "Invalid combination: prospectus fields must be both set or not set".to_owned(),
            ))?,
        }
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateUpateDataInputErrors {
    pub name: Option<ValidationError>,
    pub description: Option<ValidationError>,
    pub image_url: Option<ValidationError>,
    pub social_media_url: Option<ValidationError>,
    pub min_invest_shares: Option<ValidationError>,
    pub max_invest_shares: Option<ValidationError>,
    pub prospectus_url: Option<ValidationError>,
    pub prospectus_bytes: Option<ValidationError>,
    pub prospectus_hash: Option<ValidationError>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum ValidateDataUpdateInputsError {
    AllFieldsValidation(ValidateUpateDataInputErrors),
    NonValidation(String),
}

impl From<ValidateDataUpdateInputsError> for FrError {
    fn from(e: ValidateDataUpdateInputsError) -> Self {
        match e {
            ValidateDataUpdateInputsError::AllFieldsValidation(errors) => {
                let mut hm = HashMap::new();
                insert_if_some(&mut hm, "name", errors.name);
                insert_if_some(&mut hm, "description", errors.description);
                insert_if_some(&mut hm, "image", errors.image_url);
                insert_if_some(&mut hm, "social_media_url", errors.social_media_url);
                insert_if_some(&mut hm, "prospectus_bytes", errors.prospectus_bytes);
                insert_if_some(&mut hm, "prospectus_url", errors.prospectus_url);
                insert_if_some(&mut hm, "min_invest_shares", errors.min_invest_shares);
                insert_if_some(&mut hm, "max_invest_shares", errors.max_invest_shares);
                FrError::Validations(hm)
            }
            ValidateDataUpdateInputsError::NonValidation(msg) => FrError::Msg(msg),
        }
    }
}

fn insert_if_some(
    hm: &mut HashMap<String, ValidationError>,
    key: &str,
    value: Option<ValidationError>,
) {
    if let Some(value) = value {
        hm.insert(key.to_owned(), value);
    }
}

pub fn validate_prospectus_hash(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    match url {
        Some(url) => Ok(Some(validate_text_min_max_length(&url, 0, 200)?)),
        None => Ok(None),
    }
}
