use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::provider::pay_dao_provider::{
    PayDaoParJs, PayDaoProvider, PayDaoResJs, SubmitPayDaoParJs, SubmitPayDaoResJs,
};
use crate::{
    dependencies::funds_asset_specs, js::common::to_my_algo_tx1,
    service::str_to_algos::validate_funds_amount_input,
};
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use base::flows::pay_dao::pay_dao::pay_dao;
use base::flows::pay_dao::pay_dao::{submit_pay_dao, PayDaoSigned};
use mbase::dependencies::algod;

pub struct PayDaoProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl PayDaoProvider for PayDaoProviderDef {
    async fn txs(&self, pars: PayDaoParJs) -> Result<PayDaoResJs> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let customer_address = pars.customer_address.parse().map_err(Error::msg)?;
        let customer_escrow_address = pars.customer_escrow_address.parse().map_err(Error::msg)?;
        let amount = validate_funds_amount_input(&pars.amount, &funds_asset_specs)?;

        let to_sign = pay_dao(
            &algod,
            &customer_address,
            &customer_escrow_address,
            funds_asset_specs.id,
            amount,
        )
        .await?;

        Ok(PayDaoResJs {
            to_sign: to_my_algo_tx1(&to_sign.tx)?,
        })
    }

    async fn submit(&self, pars: SubmitPayDaoParJs) -> Result<SubmitPayDaoResJs> {
        let algod = algod();

        let res = submit_pay_dao(
            &algod,
            PayDaoSigned {
                tx: signed_js_tx_to_signed_tx1(&pars.tx)?,
            },
        )
        .await?;

        log::debug!("Submit pay dao res: {:?}", res);

        Ok(SubmitPayDaoResJs {})
    }
}
