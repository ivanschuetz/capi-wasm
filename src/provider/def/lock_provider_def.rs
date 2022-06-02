use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::provider::lock_provider::{
    LockParJs, LockProvider, LockResJs, SubmitLockParJs, SubmitLockResJs,
};
use crate::service::invest_or_lock::submit_apps_optins_from_js;
use crate::{dependencies::capi_deps, js::common::to_my_algo_txs1};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::lock::lock::{submit_lock, LockSigned};
use base::flows::{create_dao::storage::load_dao::load_dao, lock::lock::lock};
use mbase::dependencies::algod;
use mbase::models::share_amount::ShareAmount;

pub struct LockProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl LockProvider for LockProviderDef {
    async fn txs(&self, pars: LockParJs) -> Result<LockResJs> {
        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;

        let share_amount = ShareAmount::new(pars.share_count.parse()?);

        let dao = load_dao(&algod, pars.dao_id.parse()?, &api, &capi_deps).await?;

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        let to_sign = lock(
            &algod,
            investor_address,
            share_amount,
            dao.shares_asset_id,
            dao.app_id,
        )
        .await?;

        let to_sign_txs = vec![to_sign.central_app_call_setup_tx, to_sign.shares_xfer_tx];

        Ok(LockResJs {
            to_sign: to_my_algo_txs1(&to_sign_txs)?,
        })
    }

    async fn submit(&self, pars: SubmitLockParJs) -> Result<SubmitLockResJs> {
        let algod = algod();

        if let Some(app_opt_ins) = pars.app_opt_ins {
            submit_apps_optins_from_js(&algod, &app_opt_ins).await?;
        }

        // sanity check
        if pars.txs.len() != 2 {
            return Err(anyhow!("Invalid app optins count: {}", pars.txs.len()));
        }

        // lock tx group
        let central_app_call_tx = &pars.txs[0];
        let shares_xfer_tx = &pars.txs[1];

        let res = submit_lock(
            &algod,
            LockSigned {
                central_app_call_setup_tx: signed_js_tx_to_signed_tx1(central_app_call_tx)?,
                shares_xfer_tx_signed: signed_js_tx_to_signed_tx1(shares_xfer_tx)?,
            },
        )
        .await?;

        log::debug!("Submit lock res: {:?}", res);

        Ok(SubmitLockResJs {})
    }
}
