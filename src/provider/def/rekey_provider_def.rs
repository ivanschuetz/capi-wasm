use crate::dependencies::capi_deps;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::rekey_provider::{
    RekeyParJs, RekeyProvider, RekeyResJs, SubmitRekeyParJs, SubmitRekeyResJs,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::dependencies::teal_api;
use base::flows::create_dao::storage::load_dao::load_dao;
use base::flows::rekey::rekey::{rekey, submit_rekey, RekeySigned};
use mbase::dependencies::algod;

pub struct RekeyProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RekeyProvider for RekeyProviderDef {
    async fn txs(&self, pars: RekeyParJs) -> Result<RekeyResJs> {
        log::debug!("_bridge_rekey, pars: {:?}", pars);

        let algod = algod();
        let api = teal_api();
        let capi_deps = capi_deps()?;

        let dao = load_dao(&algod, pars.dao_id.parse()?, &api, &capi_deps).await?;

        let auth = pars.auth_address.parse().map_err(Error::msg)?;

        let to_sign = rekey(&algod, &dao.owner, &auth).await?;

        Ok(RekeyResJs {
            to_sign: ToSignJs::new(vec![to_sign.tx])?,
        })
    }

    async fn submit(&self, pars: SubmitRekeyParJs) -> Result<SubmitRekeyResJs> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(anyhow!("Unexpected rekey txs length: {}", pars.txs.len()));
        }

        submit_rekey(
            &algod,
            RekeySigned {
                tx: signed_js_tx_to_signed_tx1(&pars.txs[0])?,
            },
        )
        .await?;

        Ok(SubmitRekeyResJs {})
    }
}
