use algonaut::core::{Address, MicroAlgos};
use anyhow::Result;
use core::{dependencies::Env, flows::create_project::model::Project};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectForUsers {
    pub id: String,
    pub name: String,
    pub asset_price: MicroAlgos,
    pub asset_name: String,
    pub asset_supply: u64,
    pub investors_share: u64,
    pub shares_asset_id: u64,
    pub central_app_id: u64,
    pub invest_escrow_address: Address,
    pub staking_escrow_address: Address,
    pub central_escrow_address: Address,
    pub customer_escrow_address: Address,
    pub invest_link: String,
    pub my_investment_link: String,
    pub project_link: String,
    pub creator: Address,
}

pub fn project_to_project_for_users(env: &Env, project: &Project) -> Result<ProjectForUsers> {
    let project_hash_str = project.hash()?.url_str();
    Ok(ProjectForUsers {
        id: project_hash_str.clone(),
        name: project.specs.name.clone(),
        asset_price: project.specs.asset_price,
        asset_name: project.specs.shares.token_name.clone(),
        asset_supply: project.specs.shares.count,
        investors_share: project.specs.investors_share,
        shares_asset_id: project.shares_asset_id,
        central_app_id: project.central_app_id,
        invest_escrow_address: *project.invest_escrow.address(),
        staking_escrow_address: *project.staking_escrow.address(),
        central_escrow_address: *project.central_escrow.address(),
        customer_escrow_address: *project.customer_escrow.address(),
        invest_link: format!("{}/invest/{}", frontend_host(env), project_hash_str),
        my_investment_link: format!("{}/investment/{}", frontend_host(env), project_hash_str),
        project_link: format!("{}/project/{}", frontend_host(env), project_hash_str),
        creator: project.creator,
    })
}

fn frontend_host(env: &Env) -> &'static str {
    match env {
        Env::Local => "http://localhost:3000",
        Env::Test => "http://test.app.capi.money",
    }
}
