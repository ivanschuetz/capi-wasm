use crate::error::FrError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::pay_dao_provider::{
    PayDaoParJs, PayDaoProvider, PayDaoResJs, SubmitPayDaoParJs, SubmitPayDaoResJs,
};
use crate::{
    dependencies::funds_asset_specs, service::number_formats::validate_funds_amount_input,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::flows::pay_dao::pay_dao::pay_dao_app;
use base::flows::pay_dao::pay_dao::{submit_pay_dao, PayDaoSigned};
use mbase::dependencies::algod;
use mbase::models::dao_id::DaoId;

pub struct PayDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PayDaoProvider for PayDaoProviderDef {
    async fn txs(&self, pars: PayDaoParJs) -> Result<PayDaoResJs, FrError> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let customer_address = pars.customer_address.parse().map_err(Error::msg)?;
        let dao_id: DaoId = pars.dao_id.parse().map_err(Error::msg)?;
        let amount = validate_funds_amount_input(&pars.amount, &funds_asset_specs)?;

        let to_sign = pay_dao_app(
            &algod,
            &customer_address,
            dao_id.0,
            funds_asset_specs.id,
            amount,
        )
        .await?;

        Ok(PayDaoResJs {
            to_sign: ToSignJs::new(vec![to_sign.tx])?,
        })
    }

    async fn submit(&self, pars: SubmitPayDaoParJs) -> Result<SubmitPayDaoResJs, FrError> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(FrError::Internal(format!(
                "Unexpected pay dao txs length: {}",
                pars.txs.len()
            )));
        }
        let tx = &pars.txs[0];

        let res = submit_pay_dao(
            &algod,
            PayDaoSigned {
                tx: signed_js_tx_to_signed_tx1(tx)?,
            },
        )
        .await?;

        log::debug!("Submit pay dao res: {:?}", res);

        Ok(SubmitPayDaoResJs {})
    }
}
