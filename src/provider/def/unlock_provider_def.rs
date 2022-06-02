use crate::dependencies::capi_deps;
use crate::js::common::{signed_js_tx_to_signed_tx1, to_my_algo_txs1};
use crate::provider::unlock_provider::{
    SubmitUnlockParJs, SubmitUnlockResJs, UnlockParJs, UnlockProvider, UnlockResJs,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::unlock::unlock::{submit_unlock, unlock, UnlockSigned};
use mbase::dependencies::algod;
use mbase::state::dao_app_state::dao_investor_state;

pub struct UnlockProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UnlockProvider for UnlockProviderDef {
    async fn txs(&self, pars: UnlockParJs) -> Result<UnlockResJs> {
        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;

        let dao = load_dao(&algod, pars.dao_id.parse()?, &api, &capi_deps).await?;

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        let investor_state = dao_investor_state(&algod, &investor_address, dao.app_id).await?;

        log::debug!("Unlocking shares: {:?}", investor_state.shares);

        let to_sign = unlock(&algod, investor_address, dao.app_id, dao.shares_asset_id).await?;

        let to_sign_txs = vec![to_sign.central_app_optout_tx];

        Ok(UnlockResJs {
            to_sign: to_my_algo_txs1(&to_sign_txs)?,
        })
    }

    async fn submit(&self, pars: SubmitUnlockParJs) -> Result<SubmitUnlockResJs> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(anyhow!("Invalid unlock txs count: {}", pars.txs.len()));
        }
        let app_call_tx = &pars.txs[0];

        let res = submit_unlock(
            &algod,
            UnlockSigned {
                central_app_optout_tx: signed_js_tx_to_signed_tx1(app_call_tx)?,
            },
        )
        .await?;

        log::debug!("Submit unlock res: {:?}", res);

        Ok(SubmitUnlockResJs {})
    }
}
