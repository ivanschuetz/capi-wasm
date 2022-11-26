use super::{mock_dao_for_users_view_data, req_delay};
use crate::{error::FrError, model::dao_js::DaoJs, provider::dao_provider::DaoProvider};
use anyhow::Result;
use async_trait::async_trait;

pub struct DaoUserViewProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DaoProvider for DaoUserViewProviderMock {
    async fn get(&self, _: String) -> Result<DaoJs, FrError> {
        req_delay().await;

        mock_dao_for_users_view_data()
    }
}
