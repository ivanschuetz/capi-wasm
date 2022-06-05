use super::mock_tx_id;
use crate::dependencies::funds_asset_specs;
use crate::provider::def::withdraw_provider_def::withdrawal_view_data;
use crate::provider::mock::{mock_to_sign, req_delay};
use crate::provider::withdraw_provider::{
    validate_withdrawal_inputs, SubmitWithdrawParJs, SubmitWithdrawPassthroughParJs,
    SubmitWithdrawResJs, WithdrawInputsPassthroughJs, WithdrawParJs, WithdrawProvider,
    WithdrawResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct WithdrawProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WithdrawProvider for WithdrawProviderMock {
    async fn txs(&self, pars: WithdrawParJs) -> Result<WithdrawResJs> {
        log::debug!("_bridge_withdraw, pars: {:?}", pars);

        let algod = algod();

        let owner = pars.sender.parse().map_err(Error::msg)?;

        // processing these to create quickly the passthrough that has to be in the result
        let inputs_par = WithdrawInputsPassthroughJs {
            sender: pars.sender.clone(),
            withdrawal_amount: pars.withdrawal_amount.clone(),
            description: pars.description.clone(),
        };

        req_delay().await;

        Ok(WithdrawResJs {
            to_sign: mock_to_sign(&algod, &owner).await?,
            pt: SubmitWithdrawPassthroughParJs {
                maybe_drain_tx_msg_pack: None,
                maybe_capi_share_tx_msg_pack: None,
                inputs: inputs_par.clone(),
            },
        })
    }

    async fn submit(&self, pars: SubmitWithdrawParJs) -> Result<SubmitWithdrawResJs> {
        let funds_asset_specs = funds_asset_specs()?;

        // validate - for UI
        let withdrawal_inputs = validate_withdrawal_inputs(&pars.pt.inputs, &funds_asset_specs)?;

        req_delay().await;

        Ok(SubmitWithdrawResJs {
            saved_withdrawal: withdrawal_view_data(
                withdrawal_inputs.amount,
                &funds_asset_specs,
                withdrawal_inputs.description,
                "Just now".to_owned(),
                mock_tx_id().parse()?,
            ),
        })
    }
}
