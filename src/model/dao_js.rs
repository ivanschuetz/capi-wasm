use crate::{
    dependencies::FundsAssetSpecs,
    service::number_formats::{
        base_units_to_display_units_readable, base_units_to_display_units_str, format_u64_readable,
    },
};
use anyhow::{anyhow, Result};
use base::flows::create_dao::model::Dao;
use chrono::Utc;
use mbase::{models::funds::FundsAmount, state::dao_app_state::Prospectus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoJs {
    pub name: String,
    pub descr_url: Option<String>,
    pub share_supply: String,
    // TODO consider passing a msgpack version of the dao back and forth to access non-display properties in wasm,
    // regular fields should be only for display purpose - so we don't need these additional not formatted "_number" or "_algo" fields.
    pub share_supply_number: String,
    pub investors_share: String,
    pub share_asset_name: String,
    pub share_price: String,
    pub share_price_number_algo: String,
    pub shares_asset_id: String,
    pub image_url: Option<String>,
    // TODO this is optional too, make it optional everywhere
    pub social_media_url: String,
    // TODO this is optional too, make it optional everywhere
    pub homepage_url: String,
    pub app_id: String,
    pub app_address: String,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub dao_link: String,
    pub creator_address: String,
    pub raise_end_date: String,
    pub raise_min_target_number: String,
    pub raise_min_target: String,
    pub total_raisable: String,
    pub total_raisable_number: String,
    pub funds_raised: String,
    pub setup_date: String,
    pub prospectus: Option<Prospectus>,
}

pub trait ToDaoJs {
    fn to_js(&self, funds_asset_specs: &FundsAssetSpecs) -> Result<DaoJs>;
}

impl ToDaoJs for Dao {
    fn to_js(&self, funds_asset_specs: &FundsAssetSpecs) -> Result<DaoJs> {
        let dao_id_str = self.id().to_string();
        let total_raisable = FundsAmount::new(
            self.token_supply
                .val()
                .checked_mul(self.share_price.val())
                .ok_or_else(|| {
                    anyhow!(
                        "Total raisable - error mul: {:?} * {:?}",
                        self.token_supply,
                        self.share_price
                    )
                })?,
        );

        let now = Utc::now();
        let past_raise_end_date = (now.timestamp() as i128)
            .checked_sub(self.raise_end_date.0 as i128)
            .ok_or_else(|| {
                anyhow!(
                    "Invalid end date: {:?} <= now ({})",
                    self.raise_end_date,
                    now
                )
            })?
            >= 0;
        let funds_raised = past_raise_end_date && self.raised.val() >= self.raise_min_target.val();

        Ok(DaoJs {
            name: self.name.clone(),
            descr_url: self.descr_url.clone(),
            share_price: base_units_to_display_units_readable(self.share_price, funds_asset_specs)?,
            share_asset_name: self.token_name.clone(),
            share_supply: format_u64_readable(self.token_supply.val())?,
            share_supply_number: self.token_supply.val().to_string(),
            investors_share: self.investors_share.value().to_string(),
            // TODO remove and use the nft url (uncomment line below)
            // image_url,
            image_url: self.image_nft.clone().map(|nft| nft.url),
            social_media_url: self.social_media_url.clone(),
            homepage_url: self.homepage_url.clone(),
            shares_asset_id: self.shares_asset_id.to_string(),
            app_id: self.app_id.to_string(),
            app_address: self.app_id.address().to_string(),
            // invest_link: format!("/{}/invest", dao_id_str),
            // for now just the dao, because we don't have a dedicated investing view anymore and the embedded view is not linked
            invest_link: format!("/{}", dao_id_str),
            my_investment_link: format!("/{}/investment", dao_id_str),
            my_investment_link_rel: format!("investment/{}", dao_id_str),
            dao_link: format!("/{}", dao_id_str),
            creator_address: self.owner.to_string(),
            share_price_number_algo: base_units_to_display_units_str(
                self.share_price,
                funds_asset_specs,
            ),
            raise_end_date: self.raise_end_date.0.to_string(),
            raise_min_target_number: self.raise_min_target.val().to_string(),
            raise_min_target: base_units_to_display_units_readable(
                self.raise_min_target,
                funds_asset_specs,
            )?,
            total_raisable: base_units_to_display_units_readable(
                total_raisable,
                funds_asset_specs,
            )?,
            total_raisable_number: total_raisable.val().to_string(),
            funds_raised: funds_raised.to_string(),
            setup_date: self.setup_date.0.to_string(),
            prospectus: self.prospectus.clone(),
        })
    }
}
