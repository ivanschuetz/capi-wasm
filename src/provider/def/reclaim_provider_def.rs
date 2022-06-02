use crate::dependencies::capi_deps;
use crate::js::common::{signed_js_tx_to_signed_tx1, to_my_algo_txs1};
use crate::provider::reclaim_provider::{
    ReclaimParJs, ReclaimProvider, ReclaimResJs, SubmitReclaimParJs, SubmitReclaimResJs,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::reclaim::reclaim::{reclaim, submit_reclaim, ReclaimSigned};
use mbase::dependencies::algod;
use mbase::models::share_amount::ShareAmount;
use mbase::state::dao_app_state::dao_investor_state;

pub struct ReclaimProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ReclaimProvider for ReclaimProviderDef {
    async fn txs(&self, pars: ReclaimParJs) -> Result<ReclaimResJs> {
        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;

        let dao = load_dao(&algod, pars.dao_id.parse()?, &api, &capi_deps).await?;

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;
        let share_amount = ShareAmount::new(pars.share_amount.parse().map_err(Error::msg)?);

        let investor_state = dao_investor_state(&algod, &investor_address, dao.app_id).await?;

        log::debug!("Reclaiming shares: {:?}", investor_state.shares);

        let to_sign = reclaim(
            &algod,
            &investor_address,
            dao.app_id,
            dao.shares_asset_id,
            share_amount,
            dao.funds_asset_id,
        )
        .await?;

        let to_sign_txs = vec![to_sign.app_call_tx, to_sign.shares_xfer_tx];

        Ok(ReclaimResJs {
            to_sign: to_my_algo_txs1(&to_sign_txs)?,
        })
    }

    async fn submit(&self, pars: SubmitReclaimParJs) -> Result<SubmitReclaimResJs> {
        let algod = algod();

        if pars.txs.len() != 2 {
            return Err(anyhow!("Invalid reclaim txs count: {}", pars.txs.len()));
        }

        let app_call_tx = signed_js_tx_to_signed_tx1(&pars.txs[0])?;
        let shares_xfer = signed_js_tx_to_signed_tx1(&pars.txs[1])?;

        let res = submit_reclaim(
            &algod,
            &ReclaimSigned {
                app_call_tx_signed: app_call_tx,
                shares_xfer_tx_signed: shares_xfer,
            },
        )
        .await?;

        log::debug!("Submit reclaim res: {:?}", res);

        Ok(SubmitReclaimResJs {})
    }
}
