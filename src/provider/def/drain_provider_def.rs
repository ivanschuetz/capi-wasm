use crate::dependencies::{api, capi_deps, funds_asset_specs};
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::common::to_my_algo_txs1;
use crate::provider::drain_provider::{
    DrainParJs, DrainProvider, DrainResJs, SubmitDrainParJs, SubmitDrainPassthroughParJs,
    SubmitDrainResJs,
};
use crate::service::str_to_algos::microalgos_to_algos;
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::drain::drain::fetch_drain_amount_and_drain;
use base::flows::drain::drain::{submit_drain_customer_escrow, DrainCustomerEscrowSigned};
use mbase::dependencies::algod;

pub struct DrainProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DrainProvider for DrainProviderDef {
    async fn txs(&self, pars: DrainParJs) -> Result<DrainResJs> {
        let algod = algod();
        let api = api();
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id, &api, &capi_deps).await?;

        let to_sign = fetch_drain_amount_and_drain(
            &algod,
            &pars.drainer_address.parse().map_err(Error::msg)?,
            dao.app_id,
            funds_asset_specs()?.id,
            &capi_deps,
            &dao.customer_escrow.account,
        )
        .await?;

        Ok(DrainResJs {
            to_sign: to_my_algo_txs1(&vec![to_sign.app_call_tx, to_sign.capi_app_call_tx])?,
            pt: SubmitDrainPassthroughParJs {
                drain_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.drain_tx)?,
                capi_share_tx_msg_pack: rmp_serde::to_vec_named(&to_sign.capi_share_tx)?,
                dao_id: dao_id.to_string(),
            },
        })
    }

    async fn submit(&self, pars: SubmitDrainParJs) -> Result<SubmitDrainResJs> {
        let algod = algod();
        let api = api();
        let capi_deps = capi_deps()?;

        let app_call_tx = &pars.txs[0];

        let res = submit_drain_customer_escrow(
            &algod,
            &DrainCustomerEscrowSigned {
                drain_tx: rmp_serde::from_slice(&pars.pt.drain_tx_msg_pack)?,
                capi_share_tx: rmp_serde::from_slice(&pars.pt.capi_share_tx_msg_pack)?,
                capi_app_call_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
                app_call_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
            },
        )
        .await?;

        log::debug!("Submit drain res: {:?}", res);

        // TODO pass the dao from drain request, no need to fetch again here?

        let dao = load_dao(&algod, pars.pt.dao_id.parse()?, &api, &capi_deps).await?;

        // TODO (low prio) Consider just recalculating instead of new fetch

        let customer_escrow_balance = algod
            .account_information(dao.customer_escrow.address())
            .await?
            .amount;

        let app_balance = algod.account_information(&dao.app_address()).await?.amount;

        Ok(SubmitDrainResJs {
            new_customer_escrow_balance: microalgos_to_algos(customer_escrow_balance).to_string(),
            new_app_balance: microalgos_to_algos(app_balance).to_string(),
        })
    }
}
