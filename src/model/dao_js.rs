use crate::{
    dependencies::FundsAssetSpecs,
    service::number_formats::{
        base_units_to_display_units_readable, base_units_to_display_units_str, format_u64_readable,
    },
};
use anyhow::Result;
use base::flows::create_dao::model::Dao;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoJs {
    pub name: String,
    pub description_id: Option<String>,
    // TODO remove?
    pub share_supply: String,
    pub investors_share: String,
    pub share_asset_name: String,
    pub share_price: String,
    pub share_price_number_algo: String,
    pub shares_asset_id: String,
    pub image_url: Option<String>,
    // TODO this is optional too, make it optional everywhere
    pub social_media_url: String,
    pub app_id: String,
    pub customer_escrow_address: String,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub dao_link: String,
    pub creator_address: String,
}

pub trait ToDaoJs {
    fn to_js(
        &self,
        description_id: Option<String>,
        image_url: Option<String>,
        funds_asset_specs: &FundsAssetSpecs,
    ) -> Result<DaoJs>;
}

impl ToDaoJs for Dao {
    fn to_js(
        &self,
        description_id: Option<String>,
        image_url: Option<String>,
        funds_asset_specs: &FundsAssetSpecs,
    ) -> Result<DaoJs> {
        let dao_id_str = self.id().to_string();
        Ok(DaoJs {
            name: self.specs.name.clone(),
            description_id,
            share_price: base_units_to_display_units_readable(
                self.specs.share_price,
                funds_asset_specs,
            )?,
            share_asset_name: self.specs.shares.token_name.clone(),
            share_supply: format_u64_readable(self.specs.shares.supply.val())?,
            investors_share: self.specs.investors_share.value().to_string(),
            image_url,
            social_media_url: self.specs.social_media_url.clone(),
            shares_asset_id: self.shares_asset_id.to_string(),
            app_id: self.app_id.to_string(),
            customer_escrow_address: self.customer_escrow.address().to_string(),
            // invest_link: format!("/{}/invest", dao_id_str),
            // for now just the dao, because we don't have a dedicated investing view anymore and the embedded view is not linked
            invest_link: format!("/{}", dao_id_str),
            my_investment_link: format!("/{}/investment", dao_id_str),
            my_investment_link_rel: format!("investment/{}", dao_id_str),
            dao_link: format!("/{}", dao_id_str),
            creator_address: self.owner.to_string(),
            share_price_number_algo: base_units_to_display_units_str(
                self.specs.share_price,
                funds_asset_specs,
            ),
        })
    }
}
