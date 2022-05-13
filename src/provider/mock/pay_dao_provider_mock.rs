use super::{mock_js_tx, req_delay};
use crate::provider::pay_dao_provider::{
    PayDaoParJs, PayDaoProvider, PayDaoResJs, SubmitPayDaoParJs, SubmitPayDaoResJs,
};
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct PayDaoProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PayDaoProvider for PayDaoProviderMock {
    async fn txs(&self, pars: PayDaoParJs) -> Result<PayDaoResJs> {
        let algod = algod();

        let customer_address = pars.customer_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(PayDaoResJs {
            to_sign: mock_js_tx(&algod, &customer_address).await?,
        })
    }

    async fn submit(&self, _: SubmitPayDaoParJs) -> Result<SubmitPayDaoResJs> {
        req_delay().await;

        Ok(SubmitPayDaoResJs {})
    }
}
