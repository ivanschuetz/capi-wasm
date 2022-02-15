use algonaut::core::Address;
use anyhow::Result;
use core::{
    flows::create_project::{model::Project, storage::load_project::ProjectId},
    funds::FundsAmount,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectForUsers {
    pub id: String,
    pub name: String,
    pub description: String,
    pub share_price: FundsAmount,
    pub asset_name: String,
    pub asset_supply: u64,
    pub investors_share: u64,
    pub logo_url: String,
    pub social_media_url: String,
    pub shares_asset_id: u64,
    pub central_app_id: u64,
    pub invest_escrow_address: Address,
    pub staking_escrow_address: Address,
    pub central_escrow_address: Address,
    pub customer_escrow_address: Address,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub project_link: String,
    pub creator: Address,
}

pub fn project_to_project_for_users(
    project: &Project,
    project_id: &ProjectId,
) -> Result<ProjectForUsers> {
    let project_id_str = project_id.to_string();
    Ok(ProjectForUsers {
        id: project_id_str.clone(),
        name: project.specs.name.clone(),
        description: project.specs.description.clone(),
        share_price: project.specs.share_price,
        asset_name: project.specs.shares.token_name.clone(),
        asset_supply: project.specs.shares.count,
        investors_share: project.specs.investors_share,
        logo_url: project.specs.logo_url.clone(),
        social_media_url: project.specs.social_media_url.clone(),
        shares_asset_id: project.shares_asset_id,
        central_app_id: project.central_app_id,
        invest_escrow_address: *project.invest_escrow.address(),
        staking_escrow_address: *project.staking_escrow.address(),
        central_escrow_address: *project.central_escrow.address(),
        customer_escrow_address: *project.customer_escrow.address(),
        invest_link: format!("/{}/invest", project_id_str),
        my_investment_link: format!("/{}/investment", project_id_str),
        my_investment_link_rel: format!("investment/{}", project_id_str),
        project_link: format!("/{}", project_id_str),
        creator: project.creator,
    })
}
