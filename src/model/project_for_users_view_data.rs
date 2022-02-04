use crate::{
    model::project_for_users::ProjectForUsers, service::str_to_algos::microalgos_to_algos,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectForUsersViewData {
    pub name: String,
    // TODO remove?
    pub share_supply: String,
    pub share_asset_name: String,
    pub share_price: String,
    pub share_price_number_algo: String,
    pub shares_asset_id: String,
    pub logo_url: String,
    pub central_app_id: String,
    pub customer_escrow_address: String,
    pub investing_escrow_address: String,
    pub staking_escrow_address: String,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub project_link: String,
    pub creator_address: String,
}

impl From<ProjectForUsers> for ProjectForUsersViewData {
    fn from(project: ProjectForUsers) -> Self {
        ProjectForUsersViewData {
            name: project.name.clone(),
            share_supply: project.asset_supply.to_string(),
            share_asset_name: project.asset_name,
            share_price: format!("{} Algo", microalgos_to_algos(project.asset_price)),
            share_price_number_algo: microalgos_to_algos(project.asset_price).to_string(),
            logo_url: project.logo_url,
            shares_asset_id: project.shares_asset_id.to_string(),
            central_app_id: project.central_app_id.to_string(),
            customer_escrow_address: project.customer_escrow_address.to_string(),
            investing_escrow_address: project.invest_escrow_address.to_string(),
            staking_escrow_address: project.staking_escrow_address.to_string(),
            invest_link: project.invest_link.clone(),
            my_investment_link: project.my_investment_link.clone(),
            my_investment_link_rel: project.my_investment_link_rel.clone(),
            project_link: project.project_link.clone(),
            creator_address: project.creator.to_string(),
        }
    }
}
