use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WyreProvider {
    async fn reserve(&self, pars: WyreReserveParsJs) -> Result<WyreReserveResJs>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct WyreReserveParsJs {
    pub address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WyreReserveResJs {
    pub url: String,
    pub reservation: String,
}
