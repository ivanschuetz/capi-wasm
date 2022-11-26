use crate::{
    error::FrError,
    provider::my_daos_provider::{MyDaosParJs, MyDaosProvider, MyDaosResJs},
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::queries::my_daos::my_daos;
use mbase::dependencies::{algod, indexer};

pub struct MyDaosProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl MyDaosProvider for MyDaosProviderDef {
    async fn get(&self, pars: MyDaosParJs) -> Result<MyDaosResJs, FrError> {
        let algod = algod();
        let indexer = indexer();

        let address = pars.address.parse().map_err(Error::msg)?;

        let daos = my_daos(&algod, &indexer, &address).await?;

        Ok(MyDaosResJs {
            daos: daos.into_iter().map(|p| p.into()).collect(),
        })
    }
}
