use crate::js::explorer_links::explorer_tx_id_link_env;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::withdraw_provider::{
    validate_withdrawal_inputs, SubmitWithdrawParJs, SubmitWithdrawPassthroughParJs,
    SubmitWithdrawResJs, WithdrawInputsPassthroughJs, WithdrawParJs, WithdrawProvider,
    WithdrawResJs,
};
use crate::{
    dependencies::FundsAssetSpecs, provider::withdrawal_history_provider::WithdrawalViewData,
    service::number_formats::base_units_to_display_units_str,
};
use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    service::drain_if_needed::drain_if_needed_tx,
};
use crate::{
    js::common::signed_js_tx_to_signed_tx1, service::drain_if_needed::prepare_pars_and_submit_drain,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::TxId;
use base::flows::withdraw::withdraw::{submit_withdraw, WithdrawSigned};
use base::flows::{
    create_dao::storage::load_dao::load_dao,
    withdraw::withdraw::{withdraw, WithdrawalInputs},
};
use mbase::dependencies::algod;
use mbase::models::funds::FundsAmount;

pub struct WithdrawProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WithdrawProvider for WithdrawProviderDef {
    async fn txs(&self, pars: WithdrawParJs) -> Result<WithdrawResJs> {
        log::debug!("_bridge_withdraw, pars: {:?}", pars);

        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;
        let capi_deps = capi_deps()?;

        let dao = load_dao(&algod, pars.dao_id.parse()?).await?;

        let inputs_par = WithdrawInputsPassthroughJs {
            sender: pars.sender.clone(),
            withdrawal_amount: pars.withdrawal_amount.clone(),
            description: pars.description.clone(),
        };

        let validated_inputs = validate_withdrawal_inputs(&inputs_par, &funds_asset_specs)?;

        // TODO we could check balance first (enough to withdraw) but then more requests? depends on which state is more likely, think about this

        let inputs = &WithdrawalInputs {
            amount: validated_inputs.amount,
            description: validated_inputs.description,
        };

        let to_sign_for_withdrawal = withdraw(
            &algod,
            pars.sender.parse().map_err(Error::msg)?,
            inputs,
            dao.app_id,
            dao.funds_asset_id,
        )
        .await?;

        let mut to_sign = vec![to_sign_for_withdrawal.withdraw_tx];

        let maybe_to_sign_for_drain = drain_if_needed_tx(
            &algod,
            &dao,
            &pars.sender.parse().map_err(Error::msg)?,
            funds_asset_specs.id,
            &capi_deps,
        )
        .await?;
        // we append drain at the end since it's optional, so the indices of the non optional txs are fixed
        if let Some(to_sign_for_drain) = maybe_to_sign_for_drain {
            to_sign.push(to_sign_for_drain.app_call_tx);
        }

        Ok(WithdrawResJs {
            to_sign: ToSignJs::new(to_sign)?,
            pt: SubmitWithdrawPassthroughParJs {
                inputs: inputs_par.clone(),
            },
        })
    }

    async fn submit(&self, pars: SubmitWithdrawParJs) -> Result<SubmitWithdrawResJs> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let withdrawal_inputs = validate_withdrawal_inputs(&pars.pt.inputs, &funds_asset_specs)?;

        // 1 tx if only withdrawal, 2 if withdrawal with drain
        if pars.txs.len() != 1 && pars.txs.len() != 2 {
            return Err(anyhow!(
                "Unexpected withdraw txs length: {}",
                pars.txs.len()
            ));
        }

        if pars.txs.len() == 2 {
            prepare_pars_and_submit_drain(&algod, &pars.txs[1]).await?;
        }

        let withdraw_tx_id = submit_withdraw(
            &algod,
            &WithdrawSigned {
                withdraw_tx: signed_js_tx_to_signed_tx1(&pars.txs[0])?,
            },
        )
        .await?;

        log::debug!("Submit withdrawal tx id: {:?}", withdraw_tx_id);

        Ok(SubmitWithdrawResJs {
            saved_withdrawal: withdrawal_view_data(
                withdrawal_inputs.amount,
                &funds_asset_specs,
                withdrawal_inputs.description,
                "Just now".to_owned(),
                withdraw_tx_id,
            ),
        })
    }
}

pub fn withdrawal_view_data(
    amount: FundsAmount,
    funds_asset_specs: &FundsAssetSpecs,
    description: String,
    date_str: String,
    tx_id: TxId,
) -> WithdrawalViewData {
    WithdrawalViewData {
        amount: base_units_to_display_units_str(amount, funds_asset_specs),
        description,
        date: date_str,
        tx_id: tx_id.to_string(),
        tx_link: explorer_tx_id_link_env(&tx_id),
        amount_not_formatted: amount.to_string(), // microalgos
    }
}
