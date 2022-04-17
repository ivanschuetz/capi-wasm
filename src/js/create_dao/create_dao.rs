use super::submit_dao::SubmitCreateDaoPassthroughParJs;
use crate::dependencies::{api, capi_deps, funds_asset_specs, FundsAssetSpecs};
use crate::inputs_validation::ValidationError;
use crate::js::common::SignedTxFromJs;
use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, to_my_algo_txs1,
};
use crate::service::constants::PRECISION;
use crate::service::str_to_algos::validate_funds_amount_input;
use algonaut::core::Address;
use algonaut::transaction::Transaction;
use anyhow::{anyhow, Error, Result};
use core::api::api::Api;
use core::api::contract::Contract;
use core::dependencies::algod;
use core::flows::create_dao::create_dao::{Escrows, Programs};
use core::flows::create_dao::create_dao_specs::CreateDaoSpecs;
use core::flows::create_dao::setup::create_shares::{submit_create_assets, CrateDaoAssetsSigned};
use core::flows::create_dao::share_amount::ShareAmount;
use core::flows::create_dao::shares_percentage::SharesPercentage;
use core::flows::create_dao::shares_specs::SharesDistributionSpecs;
use core::flows::create_dao::{
    create_dao::create_dao_txs,
    model::{CreateDaoToSign, CreateSharesSpecs},
};
use core::funds::FundsAmount;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryInto;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

/// create daos specs + signed assets txs -> create dao result
/// submits the signed assets, creates rest of dao with generated asset ids
#[wasm_bindgen]
pub async fn bridge_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_dao, pars: {:?}", pars);
    to_bridge_res(_bridge_create_dao(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_create_dao(pars: CreateDaoParJs) -> Result<CreateDaoResJs> {
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
        central_app_approval: api.template(Contract::DaoAppApproval, last_versions.app_approval)?,
        central_app_clear: api.template(Contract::DaoAppClear, last_versions.app_clear)?,
        escrows: Escrows {
            customer_escrow: api.template(Contract::DaoCustomer, last_versions.customer_escrow)?,
            invest_escrow: api.template(Contract::DaoInvesting, last_versions.investing_escrow)?,
            locking_escrow: api.template(Contract::Daolocking, last_versions.locking_escrow)?,
        },
    };

    let to_sign = create_dao_txs(
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

    // since we've to bundle all the txs to be signed in one array (so the user has to confirm only once in myalgo)
    // but return the functions in separate groups to the core logic (so rely on indices),
    // (separate groups are needed since groups need to be executed in specific order, e.g. opt in before transferring assets)
    // we double-check length here. The other txs to be signed are in single tx fields so no need to check those.
    if to_sign.escrow_funding_txs.len() != 3 {
        return Err(anyhow!(
            "Unexpected funding txs length: {}",
            to_sign.escrow_funding_txs.len()
        ));
    }
    // double-checking total length as well, just in case
    // in the next step we also check the length of the signed txs
    let txs_to_sign = &txs_to_sign(&to_sign);
    if txs_to_sign.len() as u64 != 6 {
        return Err(anyhow!(
            "Unexpected to sign dao txs length: {}",
            txs_to_sign.len()
        ));
    }

    Ok(CreateDaoResJs {
        to_sign: to_my_algo_txs1(txs_to_sign)?,
        pt: SubmitCreateDaoPassthroughParJs {
            specs: dao_specs,
            creator: creator_address.to_string(),
            escrow_optin_signed_txs_msg_pack: rmp_serde::to_vec_named(&to_sign.optin_txs)?,
            shares_asset_id: submit_assets_res.shares_asset_id,
            invest_escrow: to_sign.invest_escrow.into(),
            locking_escrow: to_sign.locking_escrow.into(),
            customer_escrow: to_sign.customer_escrow.into(),
            app_id: submit_assets_res.app_id.0,
        },
    })
}

fn validated_inputs_to_dao_specs(inputs: ValidatedDaoInputs) -> Result<CreateDaoSpecs> {
    CreateDaoSpecs::new(
        inputs.name,
        inputs.description,
        CreateSharesSpecs {
            token_name: inputs.token_name,
            supply: inputs.share_supply,
        },
        inputs.investors_part,
        inputs.share_price,
        inputs.logo_url,
        inputs.social_media_url,
    )
}

fn txs_to_sign(res: &CreateDaoToSign) -> Vec<Transaction> {
    let mut txs = vec![res.setup_app_tx.clone()];
    for tx in &res.escrow_funding_txs {
        txs.push(tx.to_owned());
    }
    txs.push(res.xfer_shares_to_invest_escrow.clone());
    txs.push(res.fund_app_tx.clone());
    txs
}

pub fn validate_dao_inputs(
    inputs: &CreateDaoFormInputsJs,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<ValidatedDaoInputs, ValidateDaoInputsError> {
    let dao_name_res = validate_dao_name(&inputs.dao_name);
    let dao_description_res = validate_dao_description(&inputs.dao_description);
    let creator_address_res = validate_address(&inputs.creator);
    let share_supply_res = validate_share_supply(&inputs.share_count);
    let share_price_res = validate_share_price(&inputs.share_price, funds_asset_specs);
    let logo_url_res = validate_logo_url(&inputs.logo_url);
    let social_media_url_res = validate_social_media_url(&inputs.social_media_url);
    let investors_share_res = validate_investors_share(&inputs.investors_share);

    let dao_name_err = dao_name_res.clone().err();
    let dao_description_err = dao_description_res.clone().err();
    let creator_address_err = creator_address_res.clone().err();
    let share_supply_err = share_supply_res.clone().err();
    let share_price_err = share_price_res.clone().err();
    let logo_url_err = logo_url_res.clone().err();
    let social_media_url_err = social_media_url_res.clone().err();
    let investors_share_err = investors_share_res.clone().err();

    if [
        dao_name_err,
        dao_description_err,
        creator_address_err,
        share_supply_err,
        share_price_err,
        logo_url_err,
        social_media_url_err,
        investors_share_err,
    ]
    .iter()
    .any(|e| e.is_some())
    {
        let errors = CreateAssetsInputErrors {
            name: dao_name_res.err(),
            description: dao_description_res.err(),
            creator: creator_address_res.err(),
            share_supply: share_supply_res.err(),
            share_price: share_price_res.err(),
            investors_share: investors_share_res.err(),
            logo_url: logo_url_res.err(),
            social_media_url: social_media_url_res.err(),
        };
        return Err(ValidateDaoInputsError::AllFieldsValidation(errors));
    }

    // Note error handling here: these errors *should* not happen, as there are caught above.
    // this is to protect from programmatic errors - being careful, because we want to avoid crashes in WASM at any cost.
    // ideally ensure it via the compiler - couldn't find how quickly other than nesting all the validations with match which is not a great alternative.
    let dao_name = dao_name_res.map_err(|e| to_single_field_val_error("dao_name", e))?;
    let dao_description =
        dao_description_res.map_err(|e| to_single_field_val_error("dao_description", e))?;
    let creator_address =
        creator_address_res.map_err(|e| to_single_field_val_error("creator_address", e))?;
    let investors_share =
        investors_share_res.map_err(|e| to_single_field_val_error("investors_share", e))?;
    let share_supply =
        share_supply_res.map_err(|e| to_single_field_val_error("share_supply", e))?;
    let share_price = share_price_res.map_err(|e| to_single_field_val_error("share_price", e))?;
    let logo_url = logo_url_res.map_err(|e| to_single_field_val_error("logo_url", e))?;
    let social_media_url =
        social_media_url_res.map_err(|e| to_single_field_val_error("social_media_url", e))?;

    // derived from other fields
    let asset_name = generate_asset_name(&dao_name).map_err(|_| {
        ValidateDaoInputsError::NonValidation(format!(
            "Error generating asset name, based on: {dao_name}"
        ))
    })?;
    let investors_part = validate_investors_part(&investors_share, share_supply).map_err(|_| {
        ValidateDaoInputsError::NonValidation(format!(
            "Error calculating investors part, based on: {investors_share:?} and: {share_supply}"
        ))
    })?;

    if investors_part > share_supply {
        return Err(ValidateDaoInputsError::InvestorsPartIsGreaterThanShareSupply);
    }

    Ok(ValidatedDaoInputs {
        name: dao_name,
        description: dao_description,
        creator: creator_address,
        token_name: asset_name,
        share_supply,
        share_price,
        investors_part,
        logo_url,
        social_media_url,
    })
}

fn to_single_field_val_error(field_name: &str, e: ValidationError) -> ValidateDaoInputsError {
    ValidateDaoInputsError::SingleFieldValidation {
        field: field_name.to_owned(),
        error: e,
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ValidateDaoInputsError {
    AllFieldsValidation(CreateAssetsInputErrors),
    InvestorsPartIsGreaterThanShareSupply,
    SingleFieldValidation {
        field: String,
        error: ValidationError,
    },
    NonValidation(String),
}

/// Errors to be shown next to the respective input fields
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateAssetsInputErrors {
    pub name: Option<ValidationError>,
    pub description: Option<ValidationError>,
    pub creator: Option<ValidationError>,
    pub share_supply: Option<ValidationError>,
    pub share_price: Option<ValidationError>,
    pub investors_share: Option<ValidationError>,
    pub logo_url: Option<ValidationError>,
    pub social_media_url: Option<ValidationError>,
}

fn validate_dao_name(name: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(name, 2, 40)
}

fn validate_dao_description(descr: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(descr, 0, 200)
}

fn validate_address(str: &str) -> Result<Address, ValidationError> {
    str.parse().map_err(|_| ValidationError::Address)
}

fn validate_text_min_max_length(
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
            actual: share_count.to_string(),
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
            actual: value.to_string(),
        })
    } else if value > max {
        Err(ValidationError::Max {
            max: max.to_string(),
            actual: value.to_string(),
        })
    } else {
        // from here we use (0..1) percentage - 100 based is just for user friendliness
        (value / Decimal::from(100u8)).try_into().map_err(|_| {
            ValidationError::Unexpected(format!("Couldn't divide {value} by 100").to_owned())
        })
    }
}

fn validate_investors_part(
    investors_percentage: &SharesPercentage,
    shares_supply: ShareAmount,
) -> Result<ShareAmount> {
    Ok(
        // the creator's part is derived (supply - investor's part)
        SharesDistributionSpecs::from_investors_percentage(investors_percentage, shares_supply)?
            .investors(),
    )
}

fn validate_logo_url(input: &str) -> Result<String, ValidationError> {
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

fn validate_social_media_url(input: &str) -> Result<String, ValidationError> {
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

pub struct ValidatedDaoInputs {
    pub name: String,
    pub description: String,
    pub creator: Address,
    pub token_name: String,
    pub share_supply: ShareAmount,
    pub share_price: FundsAmount,
    pub investors_part: ShareAmount,
    pub logo_url: String,
    pub social_media_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDaoFormInputsJs {
    pub creator: String, // not strictly a form input ("field"), but for purpose here it can be
    pub dao_name: String,
    pub dao_description: String,
    pub share_count: String,
    pub share_price: String,
    pub investors_share: String, // percentage
    pub logo_url: String,
    pub social_media_url: String,
}

impl CreateDaoFormInputsJs {
    pub fn to_dao_specs(
        &self,
        funds_asset_specs: &FundsAssetSpecs,
    ) -> Result<CreateDaoSpecs, ValidateDaoInputsError> {
        let validated_inputs = validate_dao_inputs(self, funds_asset_specs)?;
        validated_inputs_to_dao_specs(validated_inputs)
            .map_err(|e| ValidateDaoInputsError::NonValidation(format!("Unexpected: {e:?}")))
    }
}

/// The assets creation signed transactions and the specs to create the dao
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDaoParJs {
    pub pt: CreateDaoPassthroughParJs,
    // same order as the unsigned txs were sent to JS
    pub create_assets_signed_txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDaoPassthroughParJs {
    pub inputs: CreateDaoFormInputsJs,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateDaoResJs {
    pub to_sign: Vec<Value>,
    pub pt: SubmitCreateDaoPassthroughParJs, // passthrough
}
