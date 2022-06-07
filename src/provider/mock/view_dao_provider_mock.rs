use crate::{provider::view_dao_provider::{ViewDaoParJs, ViewDaoProvider, ViewDaoResJs}, service::number_formats::format_u64_readable};
use algonaut::{core::MicroAlgos, transaction::url::LinkableTransactionBuilder};
use anyhow::Result;
use async_trait::async_trait;

use super::{mock_address, mock_dao_for_users_view_data, req_delay};

pub struct ViewDaoProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ViewDaoProvider for ViewDaoProviderMock {
    async fn get(&self, _: ViewDaoParJs) -> Result<ViewDaoResJs> {
        req_delay().await;

        let dao = mock_dao_for_users_view_data()?;
        Ok(ViewDaoResJs {
            dao,
            shares_available: format_u64_readable(10000000)?,
            investors_share: "40 %".to_owned(),
            available_funds: format_u64_readable(20000)?,
            customer_payment_deeplink: LinkableTransactionBuilder::payment(
                mock_address()?,
                MicroAlgos(0),
            )
            .build()
            .as_url()
            .to_string(),
        })
    }
}
