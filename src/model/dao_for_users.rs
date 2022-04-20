use algonaut::core::Address;
use anyhow::Result;
use core::{
    flows::create_dao::{
        model::Dao,
        share_amount::ShareAmount,
        storage::load_dao::{DaoAppId, DaoId},
    },
    funds::FundsAmount,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoForUsers {
    pub id: String,
    pub name: String,
    pub description: String,
    pub share_price: FundsAmount,
    pub asset_name: String,
    pub share_supply: ShareAmount,
    pub investors_share: ShareAmount,
    pub logo_url: String,
    pub social_media_url: String,
    pub shares_asset_id: u64,
    pub app_id: DaoAppId,
    pub customer_escrow_address: Address,
    pub invest_link: String,
    pub my_investment_link: String,
    pub my_investment_link_rel: String,
    pub dao_link: String,
    pub creator: Address,
}

pub fn dao_to_dao_for_users(dao: &Dao, dao_id: &DaoId) -> Result<DaoForUsers> {
    let dao_id_str = dao_id.to_string();
    Ok(DaoForUsers {
        id: dao_id_str.clone(),
        name: dao.specs.name.clone(),
        description: dao.specs.description.clone(),
        share_price: dao.specs.share_price,
        asset_name: dao.specs.shares.token_name.clone(),
        share_supply: dao.specs.shares.supply,
        investors_share: dao.specs.investors_part(),
        logo_url: dao.specs.logo_url.clone(),
        social_media_url: dao.specs.social_media_url.clone(),
        shares_asset_id: dao.shares_asset_id,
        app_id: dao.app_id,
        customer_escrow_address: *dao.customer_escrow.address(),
        // invest_link: format!("/{}/invest", dao_id_str),
        // for now just the dao, because we don't have a dedicated investing view anymore and the embedded view is not linked
        invest_link: format!("/{}", dao_id_str),
        my_investment_link: format!("/{}/investment", dao_id_str),
        my_investment_link_rel: format!("investment/{}", dao_id_str),
        dao_link: format!("/{}", dao_id_str),
        creator: dao.owner,
    })
}
