use crate::{
    dependencies::FundsAssetSpecs, model::project_for_users::ProjectForUsers,
    service::str_to_algos::base_units_to_display_units_str,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectForUsersViewData {
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
    pub central_app_id: String,
    pub customer_escrow_address: String,
    pub investing_escrow_address: String,
    pub locking_escrow_address: String,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub project_link: String,
    pub creator_address: String,
}

pub fn project_for_users_to_view_data(
    project: ProjectForUsers,
    funds_asset_specs: &FundsAssetSpecs,
) -> ProjectForUsersViewData {
    ProjectForUsersViewData {
        name: project.name.clone(),
        description: project.description.clone(),
        share_supply: project.asset_supply.to_string(),
        share_asset_name: project.asset_name,
        share_price: base_units_to_display_units_str(project.share_price, funds_asset_specs),
        share_price_number_algo: base_units_to_display_units_str(
            project.share_price,
            funds_asset_specs,
        ),
        logo_url: project.logo_url,
        social_media_url: project.social_media_url,
        shares_asset_id: project.shares_asset_id.to_string(),
        central_app_id: project.central_app_id.to_string(),
        customer_escrow_address: project.customer_escrow_address.to_string(),
        investing_escrow_address: project.invest_escrow_address.to_string(),
        locking_escrow_address: project.locking_escrow_address.to_string(),
        invest_link: project.invest_link.clone(),
        my_investment_link: project.my_investment_link.clone(),
        my_investment_link_rel: project.my_investment_link_rel.clone(),
        project_link: project.project_link.clone(),
        creator_address: project.creator.to_string(),
    }
}
