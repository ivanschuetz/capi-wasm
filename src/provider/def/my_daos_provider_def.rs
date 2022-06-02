use crate::{
    dependencies::capi_deps,
    provider::my_daos_provider::{MyDaosParJs, MyDaosProvider, MyDaosResJs},
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::{dependencies::teal_api, queries::my_daos::my_daos};
use mbase::dependencies::{algod, indexer};

pub struct MyDaosProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl MyDaosProvider for MyDaosProviderDef {
    async fn get(&self, pars: MyDaosParJs) -> Result<MyDaosResJs> {
        let algod = algod();
        let api = teal_api();
        let indexer = indexer();
        let capi_deps = capi_deps()?;

        let address = pars.address.parse().map_err(Error::msg)?;

        let daos = my_daos(&algod, &indexer, &address, &api, &capi_deps).await?;

        Ok(MyDaosResJs {
            daos: daos.into_iter().map(|p| p.into()).collect(),
        })
    }
}
