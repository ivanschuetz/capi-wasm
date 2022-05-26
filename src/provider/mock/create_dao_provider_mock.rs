use super::{
    mock_contract_account, mock_dao_for_users_view_data, mock_js_txs, mock_msgpack_tx, req_delay,
};
use crate::dependencies::funds_asset_specs;
use crate::provider::create_dao_provider::{
    CreateDaoParJs, CreateDaoProvider, CreateDaoRes, CreateDaoResJs, SubmitCreateDaoParJs,
    SubmitSetupDaoPassthroughParJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct CreateDaoProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateDaoProvider for CreateDaoProviderMock {
    async fn txs(&self, pars: CreateDaoParJs) -> Result<CreateDaoResJs> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let creator_address = pars.pt.inputs.creator.parse().map_err(Error::msg)?;

        // this is just (local) data validation / conversion, so ok in mock (we want to test the validation UI too)
        let dao_specs = pars.pt.inputs.to_dao_specs(&funds_asset_specs)?;

        req_delay().await;

        Ok(CreateDaoResJs {
            to_sign: mock_js_txs(&algod, &creator_address).await?,
            // note that data returned here doesn't matter to UI as it's just passthrough
            pt: SubmitSetupDaoPassthroughParJs {
                specs: dao_specs,
                creator: creator_address.to_string(),
                customer_escrow_optin_to_funds_asset_tx_msg_pack: mock_msgpack_tx(
                    &algod,
                    &creator_address,
                )
                .await?,
                shares_asset_id: 1234567890,
                customer_escrow: mock_contract_account()?,
                app_id: 121212121,
                description: Some("Test description...".to_owned()),
                compressed_image: None,
            },
        })
    }

    async fn submit(&self, _: SubmitCreateDaoParJs) -> Result<CreateDaoRes> {
        req_delay().await;

        Ok(CreateDaoRes {
            dao: mock_dao_for_users_view_data()?,
            descr_error: None,
            image_error: None,
        })
    }
}
