use crate::{server::api::Api, service::str_to_algos::microalgos_to_algos};
use algonaut::{algod::v2::Algod, model::algod::v2::Asset};
use anyhow::Result;
use core::api::model::ProjectForUsers;
use serde::{Deserialize, Serialize};

pub async fn load_project_for_users_view_data(
    api: &Api,
    algod: &Algod,
    project_id: String,
) -> Result<ProjectForUsersViewData> {
    log::debug!("load_project_view_data, id: {:?}", project_id);

    let project = api.load_project_user_view(&project_id).await?;
    load_project_for_users_view_data_with_project(algod, &project).await
}

/// Temporary common entry for project loaded with id and uuid (in the future we will use only uuid)
pub async fn load_project_for_users_view_data_with_project(
    algod: &Algod,
    project: &ProjectForUsers,
) -> Result<ProjectForUsersViewData> {
    let shares_info = asset_info(algod, project.shares_asset_id).await?;

    Ok(project_for_users_to_view_data(
        project,
        shares_info.params.total,
        shares_info.params.name.unwrap_or_else(|| "".to_owned()),
    ))
}

async fn asset_info(algod: &Algod, share_asset_id: u64) -> Result<Asset> {
    Ok(algod.asset_information(share_asset_id).await?)
}

pub async fn asset_supply(algod: &Algod, share_asset_id: u64) -> Result<u64> {
    Ok(asset_info(algod, share_asset_id).await?.params.total)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectForUsersViewData {
    pub name: String,
    pub share_supply: String,
    pub share_asset_name: String,
    pub share_price: String,
    pub share_price_number_algo: String,
    pub share_asset_id: String,
    pub central_app_id: String,
    pub customer_escrow_address: String,
    pub invest_link: String,
    pub my_investment_link: String,
    pub project_link: String,
    pub creator_address: String,
}

pub fn project_for_users_to_view_data(
    project: &ProjectForUsers,
    share_supply: u64,
    share_asset_name: String,
) -> ProjectForUsersViewData {
    ProjectForUsersViewData {
        name: project.name.clone(),
        share_supply: share_supply.to_string(),
        share_asset_name,
        share_price: format!(
            "{} Algo",
            microalgos_to_algos(project.asset_price).to_string()
        ),
        share_price_number_algo: microalgos_to_algos(project.asset_price).to_string(),
        share_asset_id: project.shares_asset_id.to_string(),
        central_app_id: project.central_app_id.to_string(),
        customer_escrow_address: project.customer_escrow_address.to_string(),
        invest_link: project.invest_link.clone(),
        my_investment_link: project.my_investment_link.clone(),
        project_link: project.project_link.clone(),
        creator_address: project.creator.to_string(),
    }
}
