use crate::{
    error::FrError,
    provider::funds_raising_provider::{
        FundsRaisingParsJs, FundsRaisingProvider, FundsRaisingResJs, FundsRaisingStateJs,
    },
    service::number_formats::format_u64_readable,
};
use anyhow::Result;
use async_trait::async_trait;

pub struct FundsRaisingProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl FundsRaisingProvider for FundsRaisingProviderMock {
    async fn data(&self, _pars: FundsRaisingParsJs) -> Result<FundsRaisingResJs, FrError> {
        Ok(FundsRaisingResJs {
            raised_number: 300_500.to_string(),
            raised: format_u64_readable(300_500)?,
            state: FundsRaisingStateJs::GoalExceeded,
            goal_exceeded_percentage: Some("40%".to_owned()),
        })
    }
}
