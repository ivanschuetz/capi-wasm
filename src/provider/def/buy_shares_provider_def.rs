use crate::{
    dependencies::{api, capi_deps, funds_asset_specs},
    js::common::{signed_js_tx_to_signed_tx1, to_my_algo_txs1},
    provider::buy_shares::{
        BuySharesProvider, InvestParJs, InvestResJs, SubmitBuySharesParJs,
        SubmitBuySharesPassthroughParJs, SubmitBuySharesResJs,
    },
    service::{invest_or_lock::submit_apps_optins_from_js, str_to_algos::validate_share_count},
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::{
    flows::{
        create_dao::storage::load_dao::load_dao,
        invest::{
            invest::{invest_txs, submit_invest},
            model::InvestSigned,
        },
    },
    network_util::wait_for_pending_transaction,
};
use mbase::dependencies::algod;

pub struct BuySharesProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl BuySharesProvider for BuySharesProviderDef {
    async fn txs(&self, pars: InvestParJs) -> Result<InvestResJs> {
        let algod = algod();
        let api = api();
        let capi_deps = capi_deps()?;

        let validated_share_amount = validate_share_count(&pars.share_count)?;

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
            funds_asset_specs()?.id,
            dao.specs.share_price,
        )
        .await?;

        let to_sign_txs = vec![
            to_sign.central_app_setup_tx,
            to_sign.payment_tx,
            to_sign.shares_asset_optin_tx,
        ];

        Ok(InvestResJs {
            to_sign: to_my_algo_txs1(&to_sign_txs).map_err(Error::msg)?,
            pt: SubmitBuySharesPassthroughParJs {
                dao_msg_pack: rmp_serde::to_vec_named(&dao)?,
            },
        })
    }

    async fn submit(&self, pars: SubmitBuySharesParJs) -> Result<SubmitBuySharesResJs> {
        let algod = algod();

        if pars.txs.len() != 3 {
            return Err(anyhow!(
                "Unexpected signed invest txs length: {}",
                pars.txs.len()
            ));
        }

        let central_app_setup_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
        let payment_tx = signed_js_tx_to_signed_tx1(&pars.txs[1])?;
        let shares_asset_optin_tx = signed_js_tx_to_signed_tx1(&pars.txs[2])?;

        let dao = rmp_serde::from_slice(&pars.pt.dao_msg_pack)?;

        let submit_res = submit_invest(
            &algod,
            &InvestSigned {
                dao,
                central_app_setup_tx,
                shares_asset_optin_tx,
                payment_tx,
            },
        )
        .await?;

        let _ = wait_for_pending_transaction(&algod, &submit_res.tx_id).await?;

        log::debug!("Submit invest res: {:?}", submit_res);

        Ok(SubmitBuySharesResJs {
            message: "Success, you bought some shares!".to_owned(),
        })
    }
}
