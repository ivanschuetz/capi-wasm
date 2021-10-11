use algonaut::{algod::v2::Algod, core::Address};
use anyhow::{anyhow, Result};

pub async fn asset_holdings(algod: &Algod, address: &Address, asset_id: u64) -> Result<u64> {
    Ok(algod
        .account_information(address)
        .await?
        .assets
        .iter()
        .find(|a| a.asset_id == asset_id)
        .ok_or({
            anyhow!(
                "Invalid app state: Address: {} doesn't have asset: {}",
                address,
                asset_id
            )
        })?
        .amount)
}
