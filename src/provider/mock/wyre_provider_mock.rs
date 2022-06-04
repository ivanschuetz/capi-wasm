use crate::provider::wyre_provider::{WyreProvider, WyreReserveResJs, WyreReserveParsJs};
use anyhow::Result;
use async_trait::async_trait;
use super::req_delay;

// not sure if this mock makes sense - we might want to use the Def impl for mocks too (so delete this), 
// to see the wyre dialog / be redirected to app after?
pub struct WyreProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WyreProvider for WyreProviderMock {
    async fn reserve(&self, _pars: WyreReserveParsJs) -> Result<WyreReserveResJs> {
        req_delay().await;

        Ok(WyreReserveResJs {
            url: "".to_owned(),
            reservation: "".to_owned(),
        })
    }
}
