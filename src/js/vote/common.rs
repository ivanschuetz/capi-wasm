use algonaut::{algod::v2::Algod, core::Address};
use anyhow::{anyhow, Result};

pub async fn asset_count(algod: &Algod, address: Address, asset_id: u64) -> Result<u64> {
    Ok(algod
        .account_information(&address)
        .await?
        .assets
        .into_iter()
        .find(|a| a.asset_id == asset_id)
        // TODO confirm that this means not opted in. Having 0 assets is valid.
        .ok_or(anyhow!(
            "Address: {} not opted in to asset: {}.",
            address,
            asset_id
        ))?
        .amount)
}
