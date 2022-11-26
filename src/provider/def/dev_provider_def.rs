use crate::error::FrError;
use crate::js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs};
use crate::js::to_sign_js::ToSignJs;
use crate::provider::create_dao_provider::validate_min_raised_target_end_date;
use anyhow::{Error, Result};
use base::dev_settings::{dev_settings, submit_dev_settings, DevSettings, DevSettingsSigned};
use base::flows::create_dao::storage::load_dao::load_dao;
use mbase::dependencies::algod;
use mbase::util::network_util::wait_for_pending_transaction;
use serde::{Deserialize, Serialize};

pub struct DevProviderDef {}

impl DevProviderDef {
    pub async fn txs(&self, pars: DevSettingsParJs) -> Result<DevSettingsResJs, FrError> {
        let algod = algod();

        let dao = load_dao(&algod, pars.dao_id.parse()?).await?;

        let sender_address = pars.sender_address.parse().map_err(Error::msg)?;

        let min_raise_target_end_date =
            validate_min_raised_target_end_date(&pars.min_raise_target_end_date)?;

        let to_sign = dev_settings(
            &algod,
            &sender_address,
            dao.app_id,
            &DevSettings {
                min_raise_target_end_date,
            },
        )
        .await?;

        let to_sign_txs = vec![to_sign.app_call_tx];

        Ok(DevSettingsResJs {
            to_sign: ToSignJs::new(to_sign_txs)?,
        })
    }

    pub async fn submit(
        &self,
        pars: SubmitDevSettingsParJs,
    ) -> Result<SubmitDevSettingsResJs, FrError> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(FrError::Internal(format!(
                "Unexpected add roadmap item txs length: {}",
                pars.txs.len()
            )));
        }
        let tx = &pars.txs[0];

        let tx_id = submit_dev_settings(
            &algod,
            &DevSettingsSigned {
                app_call_tx: signed_js_tx_to_signed_tx1(tx)?,
            },
        )
        .await?;

        log::debug!("Submit dev_settings res: {:?}", tx_id);

        let _ = wait_for_pending_transaction(&algod, &tx_id).await?;

        Ok(SubmitDevSettingsResJs {})
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DevSettingsParJs {
    pub dao_id: String,
    pub sender_address: String,
    pub min_raise_target_end_date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DevSettingsResJs {
    pub to_sign: ToSignJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitDevSettingsParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubmitDevSettingsResJs {}
