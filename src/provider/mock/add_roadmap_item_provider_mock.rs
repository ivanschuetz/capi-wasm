use super::{mock_js_tx, mock_tx_id, req_delay};
use crate::provider::add_roadmap_item_provider::{
    AddRoadmapItemParJs, AddRoadmapItemResJs, SubmitAddRoadmapItemParJs,
};
use crate::provider::add_roadmap_item_provider::{
    AddRoadmapItemProvider, SubmitAddRoadmapItemResJs,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

pub struct AddRoadmapItemProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AddRoadmapItemProvider for AddRoadmapItemProviderMock {
    async fn txs(&self, pars: AddRoadmapItemParJs) -> Result<AddRoadmapItemResJs> {
        let algod = algod();
        let dao_creator = pars.creator_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(AddRoadmapItemResJs {
            to_sign: vec![mock_js_tx(&algod, &dao_creator).await?],
        })
    }

    async fn submit(&self, _: SubmitAddRoadmapItemParJs) -> Result<SubmitAddRoadmapItemResJs> {
        req_delay().await;

        Ok(SubmitAddRoadmapItemResJs {
            tx_id: mock_tx_id(),
        })
    }
}
