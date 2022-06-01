use super::{mock_js_txs, req_delay};
use crate::{
    provider::{
        buy_shares::{
            BuySharesProvider, InvestParJs, InvestResJs, SubmitBuySharesParJs,
            SubmitBuySharesPassthroughParJs, SubmitBuySharesResJs,
            ValidationBuySharesInputsOrAnyhowError,
        },
        mock::mock_msgpack_tx,
    },
    service::number_formats::validate_share_count,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct BuySharesProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BuySharesProvider for BuySharesProviderMock {
    async fn txs(
        &self,
        pars: InvestParJs,
    ) -> Result<InvestResJs, ValidationBuySharesInputsOrAnyhowError> {
        let algod = algod();

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

        // validate to show error messages in mock
        validate_share_count(&pars.share_count)?;

        req_delay().await;

        Ok(InvestResJs {
            to_sign: mock_js_txs(&algod, investor_address).await?,
            pt: SubmitBuySharesPassthroughParJs {
                dao_msg_pack: mock_msgpack_tx(&algod, investor_address).await?,
            },
        })
    }

    async fn submit(&self, _: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs> {
        req_delay().await;

        Ok(SubmitBuySharesResJs {
            message: "Success, you bought some shares!".to_owned(),
        })
    }
}
