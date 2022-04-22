use super::{mock_dao_for_users_view_data, req_delay};
use crate::{
    model::dao_for_users_view_data::DaoForUsersViewData,
    provider::dao_user_view_provider::DaoUserViewProvider,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct DaoUserViewProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DaoUserViewProvider for DaoUserViewProviderMock {
    async fn get(&self, _: String) -> Result<DaoForUsersViewData> {
        req_delay().await;

        mock_dao_for_users_view_data()
    }
}
