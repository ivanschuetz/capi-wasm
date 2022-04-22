use super::mock_dao_for_users_view_data;
use crate::{
    model::dao_for_users_view_data::DaoForUsersViewData,
    provider::load_dao_with_id_provider::LoadDaoWithIdProvider,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct LoadDaoWithIdProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl LoadDaoWithIdProvider for LoadDaoWithIdProviderMock {
    async fn get(&self, _: String) -> Result<DaoForUsersViewData> {
        mock_dao_for_users_view_data()
    }
}
