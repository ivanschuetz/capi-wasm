use super::req_delay;
use crate::{
    error::FrError,
    provider::withdrawal_history_provider::{
        LoadWithdrawalParJs, LoadWithdrawalResJs, WithdrawalHistoryProvider,
    },
};
use anyhow::Result;
use async_trait::async_trait;

pub struct WithdrawalHistoryProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WithdrawalHistoryProvider for WithdrawalHistoryProviderMock {
    async fn get(&self, _: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs, FrError> {
        req_delay().await;

        Ok(LoadWithdrawalResJs { entries: vec![] })
    }
}
