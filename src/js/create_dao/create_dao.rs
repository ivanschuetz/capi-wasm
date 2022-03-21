use super::submit_dao::SubmitCreateDaoPassthroughParJs;
use crate::dependencies::{capi_deps, funds_asset_specs, FundsAssetSpecs};
use crate::js::common::SignedTxFromJs;
use crate::js::common::{
    parse_bridge_pars, signed_js_tx_to_signed_tx1, to_bridge_res, to_my_algo_txs1,
};
use crate::service::constants::PRECISION;
use crate::service::str_to_algos::validate_funds_amount_input;
use crate::teal;
use algonaut::core::Address;
use algonaut::transaction::Transaction;
use anyhow::{anyhow, Error, Result};
use core::dependencies::algod;
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
    let funds_asset_specs = funds_asset_specs();
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

    let to_sign = create_dao_txs(
        &algod,
        &dao_specs,
        creator_address,
        creator_address, // for now creator is owner
        submit_assets_res.shares_asset_id,
        funds_asset_specs.id,
        &teal::programs(),
        PRECISION,
        submit_assets_res.app_id,
        &capi_deps,
    )
    .await?;

    // since we've to bundle all the txs to be signed in one array (so the user has to confirm only once in myalgo)
    // but return the functions in separate groups to the core logic (so rely on indices),
    // (separate groups are needed since groups need to be executed in specific order, e.g. opt in before transferring assets)
    // we double-check length here. The other txs to be signed are in single tx fields so no need to check those.
    if to_sign.escrow_funding_txs.len() != 4 {
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
            central_escrow: to_sign.central_escrow.into(),
            customer_escrow: to_sign.customer_escrow.into(),
            central_app_id: submit_assets_res.app_id,
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
    txs
}

pub fn validate_dao_inputs(
    inputs: &CreateDaoFormInputsJs,
    funds_asset_specs: &FundsAssetSpecs,
) -> Result<ValidatedDaoInputs> {
    let dao_name = validate_dao_name(&inputs.dao_name)?;
    let dao_description = validate_dao_description(&inputs.dao_description)?;
    let asset_name = generate_asset_name(&dao_name)?;
    let creator_address = inputs.creator.parse().map_err(Error::msg)?;
    let share_supply = validate_share_supply(&inputs.share_count)?;
    let share_price = validate_share_price(&inputs.share_price, funds_asset_specs)?;
    let logo_url = validate_logo_url(&inputs.logo_url)?;
    let social_media_url = validate_social_media_url(&inputs.social_media_url)?;

    let investors_share = validate_investors_share(&inputs.investors_share)?;
    let investors_part = validate_investors_part(&investors_share, share_supply)?;

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

fn validate_dao_name(name: &str) -> Result<String> {
    validate_text_min_max_length(name, 2, 40, "Dao name")
}

fn validate_dao_description(descr: &str) -> Result<String> {
    validate_text_min_max_length(descr, 0, 200, "Dao description")
}

fn validate_text_min_max_length(
    text: &str,
    min: usize,
    max: usize,
    field_name: &str,
) -> Result<String> {
    let text = text.trim();

    let dao_name_len = text.len();
    if dao_name_len < min {
        return Err(anyhow!(
            "{field_name} must have at least {} characters. Current: {}",
            min,
            text.len()
        ));
    }
    if dao_name_len > max {
        return Err(anyhow!(
            "{field_name} must not have more than {} characters. Current: {}",
            max,
            text.len()
        ));
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

fn validate_share_supply(input: &str) -> Result<ShareAmount> {
    let share_count = input.parse()?;
    if share_count == 0 {
        return Err(anyhow!("Please enter a valid share count"));
    }
    Ok(ShareAmount::new(share_count))
}

fn validate_share_price(input: &str, funds_asset_specs: &FundsAssetSpecs) -> Result<FundsAmount> {
    validate_funds_amount_input(input, funds_asset_specs)
}

fn validate_investors_share(input: &str) -> Result<SharesPercentage> {
    let value = input.parse::<Decimal>()?;
    let min = 0u8.into();
    let max = 100u8.into();
    if value >= min && value <= max {
        // from here we use (0..1) percentage - 100 based is just for user friendliness
        (value / Decimal::from(100u8)).try_into()
    } else {
        Err(anyhow!(
            "Invalid percentage value: {value}. Must be [{min}..{max}]"
        ))
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

fn validate_logo_url(input: &str) -> Result<String> {
    let count = input.len();
    let max_chars = 100;
    if count > max_chars {
        return Err(anyhow!(
            "Logo URL must not have more than {max_chars} characters. Consider using a URL shortener."
        ));
    }
    Ok(input.to_owned())
}

fn validate_social_media_url(input: &str) -> Result<String> {
    let count = input.len();
    let max_chars = 100;
    if count > max_chars {
        return Err(anyhow!(
            "Social media URL must not have more than {max_chars} characters. Consider using a URL shortener."
        ));
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
    pub fn to_dao_specs(&self, funds_asset_specs: &FundsAssetSpecs) -> Result<CreateDaoSpecs> {
        let validated_inputs = validate_dao_inputs(self, funds_asset_specs)?;
        validated_inputs_to_dao_specs(validated_inputs)
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
