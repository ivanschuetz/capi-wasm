use crate::dependencies::{capi_deps, funds_asset_specs};
use crate::error::FrError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::drain_provider::{
    DrainParJs, DrainProvider, DrainResJs, SubmitDrainParJs, SubmitDrainPassthroughParJs,
    SubmitDrainResJs,
};
use crate::service::number_formats::microalgos_to_algos;
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::drain::drain::fetch_drain_amount_and_drain;
use base::flows::drain::drain::{submit_drain, DrainSigned};
use mbase::dependencies::algod;

pub struct DrainProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl DrainProvider for DrainProviderDef {
    async fn txs(&self, pars: DrainParJs) -> Result<DrainResJs, FrError> {
        let algod = algod();
        let capi_deps = capi_deps()?;

        let dao_id = pars.dao_id.parse()?;

        let dao = load_dao(&algod, dao_id).await?;

        let to_sign = fetch_drain_amount_and_drain(
            &algod,
            &pars.drainer_address.parse().map_err(Error::msg)?,
            dao.app_id,
            funds_asset_specs()?.id,
            &capi_deps,
        )
        .await?;

        Ok(DrainResJs {
            to_sign: ToSignJs::new(vec![to_sign.app_call_tx])?,
            pt: SubmitDrainPassthroughParJs {
                dao_id: dao_id.to_string(),
            },
        })
    }

    async fn submit(&self, pars: SubmitDrainParJs) -> Result<SubmitDrainResJs, FrError> {
        let algod = algod();

        let app_call_tx = &pars.txs[0];

        let res = submit_drain(
            &algod,
            &DrainSigned {
                app_call_tx_signed: signed_js_tx_to_signed_tx1(app_call_tx)?,
            },
        )
        .await?;

        log::debug!("Submit drain res: {:?}", res);

        // TODO pass the dao from drain request, no need to fetch again here?

        let dao = load_dao(&algod, pars.pt.dao_id.parse()?).await?;

        // TODO (low prio) Consider just recalculating instead of new fetch

        let app_balance = algod.account_information(&dao.app_address()).await?.amount;

        Ok(SubmitDrainResJs {
            new_app_balance: microalgos_to_algos(app_balance).to_string(),
        })
    }
}
