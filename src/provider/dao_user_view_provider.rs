use anyhow::Result;
use async_trait::async_trait;

use crate::model::dao_for_users_view_data::DaoForUsersViewData;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DaoUserViewProvider {
    async fn get(&self, dao_id_str: String) -> Result<DaoForUsersViewData>;
}
