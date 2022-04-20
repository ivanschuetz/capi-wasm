use crate::{
    dependencies::FundsAssetSpecs, model::dao_for_users::DaoForUsers,
    service::str_to_algos::base_units_to_display_units_str,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoForUsersViewData {
    pub name: String,
    pub description: String,
    // TODO remove?
    pub share_supply: String,
    pub share_asset_name: String,
    pub share_price: String,
    pub share_price_number_algo: String,
    pub shares_asset_id: String,
    pub logo_url: String,
    pub social_media_url: String,
    pub app_id: String,
    pub customer_escrow_address: String,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub dao_link: String,
    pub creator_address: String,
}

pub fn dao_for_users_to_view_data(
    dao: DaoForUsers,
    funds_asset_specs: &FundsAssetSpecs,
) -> DaoForUsersViewData {
    DaoForUsersViewData {
        name: dao.name.clone(),
        description: dao.description.clone(),
        share_supply: dao.share_supply.to_string(),
        share_asset_name: dao.asset_name,
        share_price: base_units_to_display_units_str(dao.share_price, funds_asset_specs),
        share_price_number_algo: base_units_to_display_units_str(
            dao.share_price,
            funds_asset_specs,
        ),
        logo_url: dao.logo_url,
        social_media_url: dao.social_media_url,
        shares_asset_id: dao.shares_asset_id.to_string(),
        app_id: dao.app_id.to_string(),
        customer_escrow_address: dao.customer_escrow_address.to_string(),
        invest_link: dao.invest_link.clone(),
        my_investment_link: dao.my_investment_link.clone(),
        my_investment_link_rel: dao.my_investment_link_rel.clone(),
        dao_link: dao.dao_link.clone(),
        creator_address: dao.creator.to_string(),
    }
}
