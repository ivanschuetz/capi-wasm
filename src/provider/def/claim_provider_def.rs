use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::claim_provider::{
    ClaimParJs, ClaimProvider, ClaimResJs, SubmitClaimParJs, SubmitClaimResJs,
};
use crate::service::drain_if_needed::{drain_if_needed_tx, prepare_pars_and_submit_drain};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::diagnostics::log_claim_diagnostics;
use base::flows::claim::claim::{claim, submit_claim, ClaimSigned};
use base::flows::create_dao::storage::load_dao::load_dao;
use base::network_util::wait_for_pending_transaction;
use mbase::dependencies::algod;

pub struct ClaimProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ClaimProvider for ClaimProviderDef {
    async fn txs(&self, pars: ClaimParJs) -> Result<ClaimResJs> {
        let algod = algod();
        let funds_asset_id = funds_asset_specs()?.id;
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id).await?;

        let investor_address = &pars.investor_address.parse().map_err(Error::msg)?;

        let to_sign_for_claim = claim(&algod, investor_address, dao.app_id, funds_asset_id).await?;

        let mut to_sign = vec![to_sign_for_claim.app_call_tx];

        let maybe_to_sign_for_drain =
            drain_if_needed_tx(&algod, &dao, investor_address, funds_asset_id, &capi_deps).await?;

        // we append drain at the end since it's optional, so the indices of the non optional txs are fixed
        if let Some(to_sign_for_drain) = maybe_to_sign_for_drain {
            to_sign.push(to_sign_for_drain.app_call_tx);
        }

        Ok(ClaimResJs {
            to_sign: ToSignJs::new(to_sign)?,
        })
    }

    async fn submit(&self, pars: SubmitClaimParJs) -> Result<SubmitClaimResJs> {
        let algod = algod();

        // 1 tx if only claim, 2 if claim + 1 drain
        if pars.txs.len() != 1 && pars.txs.len() != 2 {
            return Err(anyhow!("Unexpected claim txs length: {}", pars.txs.len()));
        }

        if pars.txs.len() == 2 {
            prepare_pars_and_submit_drain(&algod, &pars.txs[1]).await?;
        }

        let app_call_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;

        ///////////////////////////
        let dao = load_dao(&algod, pars.dao_id_for_diagnostics.parse()?).await?;

        log_claim_diagnostics(
            &algod,
            &pars
                .investor_address_for_diagnostics
                .parse()
                .map_err(Error::msg)?,
            &dao,
        )
        .await?;
        ///////////////////////////

        let claim_tx_id = submit_claim(
            &algod,
            &ClaimSigned {
                app_call_tx_signed: app_call_tx,
            },
        )
        .await?;

        log::warn!("Submit claim tx id: {:?}", claim_tx_id);
        wait_for_pending_transaction(&algod, &claim_tx_id).await?;

        Ok(SubmitClaimResJs {})
    }
}
