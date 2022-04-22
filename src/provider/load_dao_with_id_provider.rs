use crate::model::dao_for_users_view_data::DaoForUsersViewData;
use anyhow::Result;
use async_trait::async_trait;

// TODO refactor with ViewDaoProvider? - both load with id str, ViewDaoProvider result contains DaoForUsersViewData
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait LoadDaoWithIdProvider {
    async fn get(&self, id_str: String) -> Result<DaoForUsersViewData>;
}
