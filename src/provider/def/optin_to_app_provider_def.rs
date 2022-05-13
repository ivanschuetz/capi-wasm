use crate::{
    js::common::to_my_algo_txs,
    provider::optin_to_app_provider::{OptInToAppParJs, OptInToAppResJs, OptinToAppProvider},
};
use algonaut::{
    algod::v2::Algod,
    core::Address,
    transaction::{tx_group::TxGroup, Transaction},
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::flows::shared::app::optin_to_dao_app;
use mbase::{dependencies::algod, models::dao_app_id::DaoAppId};

pub struct OptinToAppProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl OptinToAppProvider for OptinToAppProviderDef {
    async fn txs(&self, pars: OptInToAppParJs) -> Result<OptInToAppResJs> {
        let algod = algod();

        if is_opted_in(
            &algod,
            pars.investor_address.parse().map_err(Error::msg)?,
            pars.app_id.parse()?,
        )
        .await?
        {
            Ok(OptInToAppResJs { to_sign: None })
            // WARNING: assumption: not opted in to central -> not opted in to all apps
            // normally this should be the case, but user can clear local state of individual apps externally or there can be bugs,
            // TODO: define behavior if app's opted in status varies and implement
            // easiest is probably to return an error ("contact support") -- returning partial opt-ins is finicky
            // and opting the user out requires an additional step on the UI to sign the txs -- this seems the best solution though
        } else {
            let optins = optin_to_all_apps(
                &algod,
                &pars.investor_address.parse().map_err(Error::msg)?,
                pars.app_id.parse()?,
            )
            .await?;

            // sanity check
            if optins.len() != 1 {
                return Err(anyhow!(
                    "Invalid generated app optins count: {}",
                    optins.len()
                ));
            }

            Ok(OptInToAppResJs {
                to_sign: Some(to_my_algo_txs(&optins).map_err(|e| Error::msg(format!("{e:?}")))?),
            })
        }
    }
}

async fn optin_to_all_apps(
    algod: &Algod,
    investor_address: &Address,
    app_id: DaoAppId,
) -> Result<Vec<Transaction>> {
    let params = algod.suggested_transaction_params().await?;
    let txs = &mut [&mut optin_to_dao_app(&params, app_id, *investor_address)?];
    TxGroup::assign_group_id(txs)?;
    Ok(txs.into_iter().map(|t| t.clone()).collect())
}

async fn is_opted_in(algod: &Algod, address: Address, app_id: u64) -> Result<bool> {
    let investor_account_infos = algod.account_information(&address).await?;

    // TODO confirm that opted in -> existing local state
    Ok(investor_account_infos
        .apps_local_state
        .iter()
        .any(|a| a.id == app_id))
}
