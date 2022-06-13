use crate::{
    dependencies::{capi_deps, funds_asset_specs},
    error::FrError,
    js::{common::signed_js_tx_to_signed_tx1, to_sign_js::ToSignJs},
    provider::buy_shares::{
        BuySharesProvider, InvestParJs, InvestResJs, SubmitBuySharesParJs,
        SubmitBuySharesPassthroughParJs, SubmitBuySharesResJs,
        ValidationBuySharesInputsOrAnyhowError,
    },
    service::{invest_or_lock::submit_apps_optins_from_js, number_formats::validate_share_amount},
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::{
    dependencies::teal_api,
    flows::{
        create_dao::storage::load_dao::load_dao,
        invest::{
            invest::{invest_txs, submit_invest},
            model::InvestSigned,
        },
    },
    network_util::wait_for_pending_transaction,
    state::account_state::asset_holdings,
};
use mbase::{dependencies::algod, models::asset_amount::AssetAmount};

pub struct BuySharesProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BuySharesProvider for BuySharesProviderDef {
    async fn txs(
        &self,
        pars: InvestParJs,
    ) -> Result<InvestResJs, ValidationBuySharesInputsOrAnyhowError> {
        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;
        let funds_asset_specs = funds_asset_specs()?;

        // TODO < available shares (maybe can be passed from frontend)
        let validated_share_amount = validate_share_amount(&pars.share_count)?;

        if let Some(app_opt_ins) = pars.app_opt_ins {
            submit_apps_optins_from_js(&algod, &app_opt_ins).await?;
        }

        log::debug!("Loading the dao...");

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        let to_sign = invest_txs(
            &algod,
            &dao,
            &pars.investor_address.parse().map_err(Error::msg)?,
            dao.app_id,
            dao.shares_asset_id,
            validated_share_amount,
            funds_asset_specs.id,
            dao.specs.share_price,
        )
        .await?;

        let to_sign_txs = vec![
            to_sign.central_app_setup_tx,
            to_sign.payment_tx,
            to_sign.shares_asset_optin_tx,
        ];

        Ok(InvestResJs {
            to_sign: ToSignJs::new(to_sign_txs)?,
            pt: SubmitBuySharesPassthroughParJs {
                dao_msg_pack: rmp_serde::to_vec_named(&dao).map_err(Error::msg)?,
            },
        })
    }

    async fn submit(&self, pars: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs, FrError> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        if pars.txs.len() != 3 {
            return Err(FrError::Msg(format!(
                "Unexpected signed invest txs length: {}",
                pars.txs.len()
            )));
        }

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;
        let buy_total_cost: u64 = pars.buy_total_cost.parse().map_err(Error::msg)?;

        let central_app_setup_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
        let payment_tx = signed_js_tx_to_signed_tx1(&pars.txs[1])?;
        let shares_asset_optin_tx = signed_js_tx_to_signed_tx1(&pars.txs[2])?;

        let dao = rmp_serde::from_slice(&pars.pt.dao_msg_pack).map_err(Error::msg)?;

        let submit_res = submit_invest(
            &algod,
            &InvestSigned {
                dao,
                central_app_setup_tx,
                shares_asset_optin_tx,
                payment_tx,
            },
        )
        .await;

        if let Some(err) = submit_res.as_ref().err() {
            if err.to_string().contains("underflow on subtracting") {
                // what the user has to buy (on-ramp) to do the transaction: the amount they tried to buy - what they have
                let holdings =
                    asset_holdings(&algod, &investor_address, funds_asset_specs.id.0).await?;
                let to_buy = AssetAmount(
                    buy_total_cost
                        .checked_sub(holdings.0)
                        .ok_or(anyhow!("Error subtracting: {buy_total_cost} - {holdings}"))?,
                );
                return Err(FrError::NotEnoughFundsAsset { to_buy });
            }
        }
        let submit_res = submit_res?;

        let _ = wait_for_pending_transaction(&algod, &submit_res.tx_id).await?;

        log::debug!("Submit invest res: {:?}", submit_res);

        Ok(SubmitBuySharesResJs {
            message: "Success, you bought some shares!".to_owned(),
        })
    }
}
