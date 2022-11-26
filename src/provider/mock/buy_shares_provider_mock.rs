use super::{mock_to_sign, req_delay};
use crate::{
    error::FrError,
    provider::{
        buy_shares::{
            BuySharesProvider, InvestParJs, InvestResJs, SubmitBuySharesParJs,
            SubmitBuySharesPassthroughParJs, SubmitBuySharesResJs,
        },
        mock::mock_msgpack_tx,
    },
    service::number_formats::validate_share_amount_positive,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct BuySharesProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BuySharesProvider for BuySharesProviderMock {
    async fn txs(&self, pars: InvestParJs) -> Result<InvestResJs, FrError> {
        let algod = algod();

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

        // validate to show error messages in mock
        validate_share_amount_positive(&pars.share_count)?;

        req_delay().await;

        Ok(InvestResJs {
            to_sign: mock_to_sign(&algod, investor_address).await?,
            pt: SubmitBuySharesPassthroughParJs {
                dao_msg_pack: mock_msgpack_tx(&algod, investor_address).await?,
            },
        })
    }

    async fn submit(&self, _pars: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs, FrError> {
        req_delay().await;

        Ok(SubmitBuySharesResJs {
            message: "Success, you bought some shares!".to_owned(),
        })
    }
}
