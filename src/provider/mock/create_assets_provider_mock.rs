use super::{mock_js_txs, req_delay};
use crate::dependencies::funds_asset_specs;
use crate::provider::create_assets_provider::{
    CreateAssetsProvider, CreateDaoAssetsParJs, CreateDaoAssetsResJs,
};
use crate::provider::create_dao_provider::CreateDaoPassthroughParJs;
use crate::provider::create_dao_provider::{validate_dao_inputs, ValidationDaoInputsOrAnyhowError};
use anyhow::Result;
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct CreateAssetsProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl CreateAssetsProvider for CreateAssetsProviderMock {
    async fn txs(
        &self,
        pars: CreateDaoAssetsParJs,
    ) -> Result<CreateDaoAssetsResJs, ValidationDaoInputsOrAnyhowError> {
        let algod = algod();

        let funds_asset_specs = funds_asset_specs()?;

        // this is a mock, but we validate, to be able to see the validation UI
        let validated_inputs = validate_dao_inputs(&pars.inputs, &funds_asset_specs)?;

        req_delay().await;

        Ok(CreateDaoAssetsResJs {
            to_sign: mock_js_txs(&algod, &validated_inputs.creator).await?,
            pt: CreateDaoPassthroughParJs {
                inputs: pars.inputs,
            },
        })
    }
}
