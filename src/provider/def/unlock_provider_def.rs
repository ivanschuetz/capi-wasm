use crate::error::FrError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::unlock_provider::{
    SubmitUnlockParJs, SubmitUnlockResJs, UnlockParJs, UnlockProvider, UnlockResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::unlock::unlock::{submit_unlock, unlock, UnlockSigned};
use mbase::dependencies::algod;
use mbase::state::dao_app_state::dao_investor_state;
use mbase::util::network_util::wait_for_pending_transaction;

pub struct UnlockProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl UnlockProvider for UnlockProviderDef {
    async fn txs(&self, pars: UnlockParJs) -> Result<UnlockResJs, FrError> {
        let algod = algod();

        let dao = load_dao(&algod, pars.dao_id.parse()?).await?;

        let investor_address = pars.investor_address.parse().map_err(Error::msg)?;

        let investor_state = dao_investor_state(&algod, &investor_address, dao.app_id).await?;

        log::debug!("Unlocking shares: {:?}", investor_state.shares);

        let to_sign = unlock(&algod, investor_address, dao.app_id, dao.shares_asset_id).await?;

        let to_sign_txs = vec![to_sign.central_app_optout_tx];

        Ok(UnlockResJs {
            to_sign: ToSignJs::new(to_sign_txs)?,
        })
    }

    async fn submit(&self, pars: SubmitUnlockParJs) -> Result<SubmitUnlockResJs, FrError> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(FrError::Internal(format!(
                "Invalid unlock txs count: {}",
                pars.txs.len()
            )));
        }
        let app_call_tx = &pars.txs[0];

        let tx_id = submit_unlock(
            &algod,
            UnlockSigned {
                central_app_optout_tx: signed_js_tx_to_signed_tx1(app_call_tx)?,
            },
        )
        .await?;

        log::debug!("Submit unlock res: {:?}", tx_id);

        let _ = wait_for_pending_transaction(&algod, &tx_id).await?;

        Ok(SubmitUnlockResJs {})
    }
}
