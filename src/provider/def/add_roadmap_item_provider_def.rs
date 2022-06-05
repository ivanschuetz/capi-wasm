use std::convert::TryInto;

use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::add_roadmap_item_provider::{
    AddRoadmapItemParJs, AddRoadmapItemResJs, SubmitAddRoadmapItemParJs,
};
use crate::provider::add_roadmap_item_provider::{
    AddRoadmapItemProvider, SubmitAddRoadmapItemResJs,
};
use algonaut::crypto::HashDigest;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::roadmap::add_roadmap_item::{
    add_roadmap_item, submit_add_roadmap_item, AddRoadmapItemToSigned, RoadmapItemInputs,
};
use data_encoding::BASE64;
use mbase::date_util::timestamp_seconds_to_date;
use mbase::dependencies::algod;

pub struct AddRoadmapItemProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl AddRoadmapItemProvider for AddRoadmapItemProviderDef {
    async fn txs(&self, pars: AddRoadmapItemParJs) -> Result<AddRoadmapItemResJs> {
        let algod = algod();

        let dao_creator = pars.creator_address.parse().map_err(Error::msg)?;
        let dao_id = pars.dao_id.parse()?;

        let parent_hash = hash_str_option_to_hash_option(pars.parent)?;

        let date = timestamp_seconds_to_date(pars.date.parse()?)?;

        let to_sign = add_roadmap_item(
            &algod,
            &dao_creator,
            &RoadmapItemInputs {
                dao_id,
                title: pars.title,
                parent: Box::new(parent_hash),
                date,
            },
        )
        .await?;

        Ok(AddRoadmapItemResJs {
            to_sign: ToSignJs::new(vec![to_sign.tx])?,
        })
    }

    async fn submit(&self, pars: SubmitAddRoadmapItemParJs) -> Result<SubmitAddRoadmapItemResJs> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(anyhow!(
                "Unexpected add roadmap item txs length: {}",
                pars.txs.len()
            ));
        }
        let tx = &pars.txs[0];

        let add_roadmap_item_signed_tx = signed_js_tx_to_signed_tx1(&tx)?;

        let tx_id = submit_add_roadmap_item(
            &algod,
            &AddRoadmapItemToSigned {
                tx: add_roadmap_item_signed_tx,
            },
        )
        .await?;

        Ok(SubmitAddRoadmapItemResJs { tx_id })
    }
}

fn hash_str_option_to_hash_option(hash_str: Option<String>) -> Result<Option<HashDigest>> {
    Ok(match &hash_str {
        Some(hash_str) => {
            let bytes = BASE64.decode(hash_str.as_bytes())?;
            Some(HashDigest(bytes.clone().try_into().map_err(|e| {
                anyhow!(
                    "Couldn't convert bytes(len: {:?}): {:?} into hash. e: {:?}",
                    bytes.len(),
                    bytes,
                    e
                )
            })?))
        }
        None => None,
    })
}
