use crate::dependencies::FundsAssetSpecs;
use crate::error::FrError;
use crate::inputs_validation::ValidationError;
use crate::js::bridge::log_wrap_new;
use crate::js::common::SignedTxFromJs;
use crate::js::to_sign_js::ToSignJs;
use crate::model::dao_js::DaoJs;
use crate::service::number_formats::validate_funds_amount_input;
use algonaut::core::Address;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use mbase::models::create_shares_specs::CreateSharesSpecs;
use mbase::models::funds::FundsAmount;
use mbase::models::setup_dao_specs::SetupDaoSpecs;
use mbase::models::share_amount::ShareAmount;
use mbase::models::shares_percentage::SharesPercentage;
use mbase::models::timestamp::Timestamp;
use mbase::state::dao_app_state::Prospectus;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt::Debug;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateDaoProvider {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs, FrError>;
    /// create daos specs + signed assets txs -> create dao result
    /// submits the signed assets, creates rest of dao with generated asset ids
    async fn submit(&self, pars: SubmitCreateDaoParJs) -> Result<CreateDaoRes, FrError>;
}

pub struct ValidatedDaoInputs {
    pub name: String,
    pub description_url: Option<String>,
    pub creator: Address,
    pub token_name: String,
    pub share_supply: ShareAmount,
    pub shares_for_investors: ShareAmount,
    pub share_price: FundsAmount,
    pub investors_share: SharesPercentage,
    pub image_url: Option<String>,
    pub social_media_url: String,
    pub min_raise_target: FundsAmount,
    pub min_raise_target_end_date: Timestamp,
    pub prospectus_url: Option<String>,
    pub prospectus_bytes: Option<Vec<u8>>,
    pub min_invest_amount: ShareAmount,
    pub max_invest_amount: ShareAmount,
}

impl ValidatedDaoInputs {
    pub fn prospectus_data(&self) -> Result<Option<(String, Vec<u8>)>> {
        match (&self.prospectus_url, &self.prospectus_bytes) {
            (Some(url), Some(bytes)) => Ok(Some((url.clone(), bytes.clone()))),
            (None, None) => Ok(None),
            _ => Err(anyhow!(
                "Invalid state: prospectus url and hash should both be set or none"
            )),
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CreateDaoFormInputsJs {
    pub creator: String, // not strictly a form input ("field"), but for purpose here it can be
    pub dao_name: String,
    pub dao_descr_url: Option<String>,
    pub share_count: String,
    pub shares_for_investors: String,
    pub share_price: String,
    pub investors_share: String, // percentage (0..100), with decimals (max decimals number defined in validations)
    pub image_url: Option<String>,
    pub social_media_url: String,
    pub min_raise_target: String,
    pub min_raise_target_end_date: String,
    pub prospectus_url: Option<String>,
    pub prospectus_bytes: Option<Vec<u8>>,
    pub min_invest_amount: String,
    pub max_invest_amount: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CreateDaoRes {
    pub dao: DaoJs,
}

impl CreateDaoFormInputsJs {
    pub fn to_dao_specs(
        &self,
        funds_asset_specs: &FundsAssetSpecs,
    ) -> Result<SetupDaoSpecs, ValidateDaoInputsError> {
        let validated_inputs = validate_dao_inputs(self, funds_asset_specs)?;
        validated_inputs_to_dao_specs(&validated_inputs)
            .map_err(|e| ValidateDaoInputsError::NonValidation(format!("Unexpected: {e}")))
    }
}

pub fn validated_inputs_to_dao_specs(inputs: &ValidatedDaoInputs) -> Result<SetupDaoSpecs> {
    let prospectus = inputs
        .prospectus_data()?
        .map(|(url, bytes)| Prospectus::new(&bytes, url));

    SetupDaoSpecs::new(
        inputs.name.clone(),
        inputs.description_url.clone(),
        CreateSharesSpecs {
            token_name: inputs.token_name.clone(),
            supply: inputs.share_supply,
        },
        inputs.investors_share,
        inputs.share_price,
        inputs.image_url.clone(),
        inputs.social_media_url.clone(),
        inputs.shares_for_investors,
        inputs.min_raise_target,
        inputs.min_raise_target_end_date,
        prospectus,
        inputs.min_invest_amount,
        inputs.max_invest_amount,
    )
}

pub fn validate_dao_inputs(
    inputs: &CreateDaoFormInputsJs,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<ValidatedDaoInputs, ValidateDaoInputsError> {
    let dao_name_res = validate_dao_name(&inputs.dao_name);
    let dao_description_url_res = validate_dao_description_url_opt(&inputs.dao_descr_url);
    let creator_address_res = validate_address(&inputs.creator);
    let share_supply_res = validate_share_supply(&inputs.share_count);
    let shares_for_investors_res = validate_shares_for_investors(&inputs.shares_for_investors);
    let share_price_res = validate_share_price(&inputs.share_price, funds_asset_specs);
    let image_url_res = validate_image_url(&inputs.image_url);
    let social_media_url_res = validate_social_media_url(&inputs.social_media_url);
    let investors_share_res = validate_investors_share(&inputs.investors_share);
    let min_raise_target_res =
        validate_min_raised_target(&inputs.min_raise_target, funds_asset_specs);
    let min_raised_target_end_date_res =
        validate_min_raised_target_end_date(&inputs.min_raise_target_end_date);
    let prospectus_url_res = validate_prospectus_url(&inputs.prospectus_url);
    let prospectus_bytes_res = validate_prospectus_bytes(&inputs.prospectus_bytes);
    let min_invest_amount_res = validate_min_invest_amount(&inputs.min_invest_amount);
    let max_invest_amount_res = validate_max_invest_amount(&inputs.max_invest_amount);

    match (
        &dao_name_res,
        &dao_description_url_res,
        &creator_address_res,
        &share_supply_res,
        &shares_for_investors_res,
        &share_price_res,
        &image_url_res,
        &social_media_url_res,
        &investors_share_res,
        &min_raise_target_res,
        &min_raised_target_end_date_res,
        &prospectus_url_res,
        &prospectus_bytes_res,
        &min_invest_amount_res,
        &max_invest_amount_res,
    ) {
        (
            Ok(dao_name),
            Ok(dao_descr),
            Ok(creator_address),
            Ok(share_supply),
            Ok(shares_for_investors),
            Ok(share_price),
            Ok(image_url),
            Ok(social_media_url),
            Ok(investors_share),
            Ok(min_raise_target),
            Ok(min_raise_target_end_date),
            Ok(prospectus_url),
            Ok(prospectus_bytes),
            Ok(min_invest_amount),
            Ok(max_invest_amount),
        ) => {
            // derived from other fields
            let asset_name = generate_asset_name(dao_name).map_err(|_| {
                ValidateDaoInputsError::NonValidation(format!(
                    "Error generating asset name, based on: {dao_name}"
                ))
            })?;

            // TODO should these check available shares instead of supply?

            if shares_for_investors > share_supply {
                let errors = CreateAssetsInputErrors {
                    shares_for_investors: Some(
                        ValidationError::SharesForInvestorsGreaterThanSupply,
                    ),
                    ..CreateAssetsInputErrors::default()
                };
                return Err(ValidateDaoInputsError::AllFieldsValidation(errors));
            }

            if min_invest_amount > share_supply {
                let errors = CreateAssetsInputErrors {
                    min_invest_amount: Some(ValidationError::ShareCountLargerThanAvailable),
                    ..CreateAssetsInputErrors::default()
                };
                return Err(ValidateDaoInputsError::AllFieldsValidation(errors));
            }

            if max_invest_amount > share_supply {
                let errors = CreateAssetsInputErrors {
                    max_invest_amount: Some(ValidationError::ShareCountLargerThanAvailable),
                    ..CreateAssetsInputErrors::default()
                };
                return Err(ValidateDaoInputsError::AllFieldsValidation(errors));
            }

            if min_invest_amount > max_invest_amount {
                let errors = CreateAssetsInputErrors {
                    min_invest_amount: Some(ValidationError::MustBeLessThanMaxInvestAmount),
                    max_invest_amount: Some(ValidationError::MustBeGreaterThanMinInvestAmount),
                    ..CreateAssetsInputErrors::default()
                };
                return Err(ValidateDaoInputsError::AllFieldsValidation(errors));
            }

            Ok(ValidatedDaoInputs {
                name: dao_name.clone(),
                description_url: dao_descr.clone(),
                creator: *creator_address,
                token_name: asset_name,
                share_supply: *share_supply,
                shares_for_investors: *shares_for_investors,
                share_price: *share_price,
                investors_share: *investors_share,
                social_media_url: social_media_url.clone(),
                min_raise_target: *min_raise_target,
                min_raise_target_end_date: *min_raise_target_end_date,
                image_url: image_url.clone(),
                prospectus_url: prospectus_url.clone(),
                prospectus_bytes: prospectus_bytes.clone(),
                min_invest_amount: *min_invest_amount,
                max_invest_amount: *max_invest_amount,
            })
        }
        _ => Err(ValidateDaoInputsError::AllFieldsValidation(
            CreateAssetsInputErrors {
                name: dao_name_res.err(),
                description: dao_description_url_res.err(),
                creator: creator_address_res.err(),
                share_supply: share_supply_res.err(),
                shares_for_investors: shares_for_investors_res.err(),
                share_price: share_price_res.err(),
                image_url: image_url_res.err(),
                social_media_url: social_media_url_res.err(),
                investors_share: investors_share_res.err(),
                min_raise_target: min_raise_target_res.err(),
                min_raise_target_end_date: min_raised_target_end_date_res.err(),
                prospectus_url: prospectus_url_res.err(),
                prospectus_bytes: prospectus_bytes_res.err(),
                min_invest_amount: min_invest_amount_res.err(),
                max_invest_amount: max_invest_amount_res.err(),
            },
        )),
    }
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct CreateDaoParJs {
    pub pt: CreateDaoPassthroughParJs,
    // same order as the unsigned txs were sent to JS
    pub create_assets_signed_txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct CreateDaoPassthroughParJs {
    pub inputs: CreateDaoFormInputsJs,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct CreateDaoResJs {
    pub to_sign: ToSignJs,
    pub pt: SubmitSetupDaoPassthroughParJs, // passthrough
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum ValidateDaoInputsError {
    AllFieldsValidation(CreateAssetsInputErrors),
    NonValidation(String),
}

/// Errors to be shown next to the respective input fields
#[derive(Tsify, Debug, Clone, Serialize, Default)]
#[tsify(into_wasm_abi)]
pub struct CreateAssetsInputErrors {
    pub name: Option<ValidationError>,
    pub description: Option<ValidationError>,
    pub creator: Option<ValidationError>,
    pub share_supply: Option<ValidationError>,
    pub shares_for_investors: Option<ValidationError>,
    pub share_price: Option<ValidationError>,
    pub investors_share: Option<ValidationError>,
    pub image_url: Option<ValidationError>,
    pub social_media_url: Option<ValidationError>,
    pub min_raise_target: Option<ValidationError>,
    pub min_raise_target_end_date: Option<ValidationError>,
    pub prospectus_url: Option<ValidationError>,
    pub prospectus_bytes: Option<ValidationError>,
    pub min_invest_amount: Option<ValidationError>,
    pub max_invest_amount: Option<ValidationError>,
}

pub fn validate_dao_name(name: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(name, 2, 40)
}

pub fn validate_dao_description_url_opt(
    descr: &Option<String>,
) -> Result<Option<String>, ValidationError> {
    match descr {
        Some(d) => Ok(Some(validate_dao_description_url(d)?)),
        None => Ok(None),
    }
}

/// Note: validation of the description itself is separate (currently in js - could be done in wasm, just isolated),
/// we upload to ipfs in js because of the js web3.storage library we're using
/// and we prefer to keep this in js, as with white label we've to keep these kind of services/libraries replaceable to operators.
fn validate_dao_description_url(descr: &str) -> Result<String, ValidationError> {
    // 150 is just a reasonable upper bound (a bit more than 2x of an IPFS url using ipfs.io gateway)
    validate_text_min_max_length(descr, 0, 150)
}

pub fn validate_address(str: &str) -> Result<Address, ValidationError> {
    str.parse().map_err(|_| ValidationError::Address)
}

pub fn validate_text_min_max_length(
    text: &str,
    min: usize,
    max: usize,
) -> Result<String, ValidationError> {
    let text = text.trim();

    let dao_name_len = text.len();
    if dao_name_len < min {
        return Err(ValidationError::MinLength {
            min: min.to_string(),
            actual: dao_name_len.to_string(),
        });
    }
    if dao_name_len > max {
        return Err(ValidationError::MaxLength {
            max: max.to_string(),
            actual: dao_name_len.to_string(),
        });
    }

    Ok(text.to_owned())
}

pub fn validate_image_url(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    match url {
        Some(url) => Ok(Some(validate_text_min_max_length(url, 0, 200)?)),
        None => Ok(None),
    }
}

pub fn validate_prospectus_url(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    match url {
        Some(url) => Ok(Some(validate_text_min_max_length(url, 0, 200)?)),
        None => Ok(None),
    }
}

pub fn validate_prospectus_bytes(
    bytes: &Option<Vec<u8>>,
) -> Result<Option<Vec<u8>>, ValidationError> {
    match bytes {
        Some(bytes) => {
            if bytes.is_empty() {
                Err(ValidationError::Unexpected(
                    "Prospectus bytes must be either None or not empty".to_owned(),
                ))
            } else {
                Ok(Some(bytes.clone()))
            }
        }
        None => Ok(None),
    }
}

fn generate_asset_name(validated_dao_name: &str) -> Result<String> {
    let mut asset_name = validated_dao_name;
    let asset_name_max_length = 7;
    if validated_dao_name.len() > asset_name_max_length {
        asset_name = &asset_name[0..asset_name_max_length];
    }
    Ok(asset_name.to_owned())
}

fn validate_share_supply(input: &str) -> Result<ShareAmount, ValidationError> {
    let share_count: u64 = input.parse().map_err(|_| ValidationError::NotAnInteger)?;
    if share_count == 0 {
        return Err(ValidationError::Min {
            min: 1u8.to_string(),
        });
    }
    Ok(ShareAmount::new(share_count))
}

fn validate_shares_for_investors(input: &str) -> Result<ShareAmount, ValidationError> {
    let share_count: u64 = input.parse().map_err(|_| ValidationError::NotAnInteger)?;
    if share_count == 0 {
        return Err(ValidationError::Min {
            min: 1u8.to_string(),
        });
    }
    Ok(ShareAmount::new(share_count))
}

pub fn validate_min_invest_amount(input: &str) -> Result<ShareAmount, ValidationError> {
    let share_count: u64 = input.parse().map_err(|_| ValidationError::NotAnInteger)?;
    if share_count == 0 {
        return Err(ValidationError::Min {
            min: 1u8.to_string(),
        });
    }
    Ok(ShareAmount::new(share_count))
}

pub fn validate_max_invest_amount(input: &str) -> Result<ShareAmount, ValidationError> {
    let share_count: u64 = input.parse().map_err(|_| ValidationError::NotAnInteger)?;
    if share_count == 0 {
        return Err(ValidationError::Min {
            min: 1u8.to_string(),
        });
    }
    Ok(ShareAmount::new(share_count))
}

fn validate_share_price(
    input: &str,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<FundsAmount, ValidationError> {
    validate_funds_amount_input(input, funds_asset_specs)
}

fn validate_investors_share(input: &str) -> Result<SharesPercentage, ValidationError> {
    let value = input
        .parse::<Decimal>()
        .map_err(|_| ValidationError::NotADecimal)?;

    let min = 0u8.into();
    let max = 100u8.into();

    if value < min {
        Err(ValidationError::Min {
            min: min.to_string(),
        })
    } else if value > max {
        Err(ValidationError::Max {
            max: max.to_string(),
        })
    } else {
        // from here we use (0..1) percentage - 100 based is just for user friendliness
        (value / Decimal::from(100u8))
            .try_into()
            .map_err(|_| ValidationError::Unexpected(format!("Couldn't divide {value} by 100")))
    }
}

pub fn validate_social_media_url(input: &str) -> Result<String, ValidationError> {
    let count = input.len();
    let max_chars = 100;
    if count > max_chars {
        return Err(ValidationError::MaxLength {
            max: max_chars.to_string(),
            actual: count.to_string(),
        });
    }
    Ok(input.to_owned())
}

fn validate_min_raised_target(
    input: &str,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<FundsAmount, ValidationError> {
    validate_funds_amount_input(input, funds_asset_specs)
}

pub fn validate_min_raised_target_end_date(input: &str) -> Result<Timestamp, ValidationError> {
    let timestamp = Timestamp(input.parse().map_err(|_| ValidationError::NotTimestamp)?);
    // we'll treat invalid conversion to date, as invalid timestamp
    // this comes from casting to i64, which is required by NaiveDateTime
    let date = timestamp
        .to_date()
        .map_err(|_| ValidationError::NotTimestamp)?;

    if date <= Utc::now() {
        return Err(ValidationError::MustBeAfterNow);
    }
    Ok(timestamp)
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitCreateDaoParJs {
    pub txs: Vec<SignedTxFromJs>,
    pub pt: SubmitSetupDaoPassthroughParJs, // passthrough
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSetupDaoPassthroughParJs {
    pub specs: SetupDaoSpecs,
    pub creator: String,
    pub shares_asset_id: u64,
    pub app_id: u64,
    pub description_url: Option<String>,
    pub setup_date: String,
}

#[wasm_bindgen(js_name=createDao)]
pub async fn create_dao(pars: CreateDaoParJs) -> Result<CreateDaoResJs, FrError> {
    log_wrap_new("create_dao", pars, async move |pars| {
        providers()?.create_dao.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitCreateDao)]
pub async fn submit_create_dao(pars: SubmitCreateDaoParJs) -> Result<CreateDaoRes, FrError> {
    log_wrap_new("submit_create_dao", pars, async move |pars| {
        providers()?.create_dao.submit(pars).await
    })
    .await
}
