use algonaut::core::Address;
use anyhow::{anyhow, Error, Result};
use core::{
    capi_asset::{
        capi_app_id::CapiAppId, capi_asset_dao_specs::CapiAssetDaoDeps, capi_asset_id::CapiAssetId,
    },
    dependencies::{network, Network},
    funds::FundsAssetId,
};
use rust_decimal::Decimal;
use std::{convert::TryInto, str::FromStr};

/// URL determined by environment variable
pub fn explorer_base_url<'a>() -> &'a str {
    explorer_base_url_for_net(&network())
}

pub fn explorer_base_url_for_net<'a>(net: &Network) -> &'a str {
    match net {
        // We don't have an explorer for private network - this will not find anything - so we just test that it opens the explorer and searches
        Network::Private | Network::SandboxPrivate | Network::Test => {
            "https://testnet.algoexplorer.io/"
        }
    }
}

pub fn funds_asset_specs() -> Result<FundsAssetSpecs> {
    Ok(FundsAssetSpecs {
        id: funds_asset_id()?,
        decimals: 6,
    })
}

pub fn capi_deps() -> Result<CapiAssetDaoDeps> {
    Ok(CapiAssetDaoDeps {
        escrow: capi_escrow_address()?,
        escrow_percentage: Decimal::from_str("0.01")?.try_into()?,
        app_id: capi_app_id()?,
        asset_id: capi_asset_id()?,
    })
}

/// This is WASM-only as the decimals are needed only for formatting - we don't need this in core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FundsAssetSpecs {
    pub id: FundsAssetId,
    // Technically (algonaut) decimals is u64, but Decimal wants u32 and realistically u32 is enough. + we currently hardcode this.
    pub decimals: u32,
}

pub fn funds_asset_id() -> Result<FundsAssetId> {
    let str = option_env!("FUNDS_ASSET_ID").ok_or_else(|| anyhow!("Please pass FUNDS_ASSET_ID"))?;
    log::debug!("Funds asset id: {:?}", str);

    Ok(FundsAssetId(str.parse().map_err(Error::msg)?))
}

pub fn capi_escrow_address() -> Result<Address> {
    let str = option_env!("CAPI_ESCROW_ADDRESS")
        .ok_or_else(|| anyhow!("Please pass CAPI_ESCROW_ADDRESS"))?;
    log::debug!("Capi escrow address: {:?}", str);

    str.parse().map_err(Error::msg)
}

pub fn capi_app_id() -> Result<CapiAppId> {
    let str = option_env!("CAPI_APP_ID").ok_or_else(|| anyhow!("Please pass CAPI_APP_ID"))?;
    log::debug!("Capi app id: {:?}", str);

    Ok(CapiAppId(str.parse().map_err(Error::msg)?))
}

pub fn capi_asset_id() -> Result<CapiAssetId> {
    let str = option_env!("CAPI_ASSET_ID").ok_or_else(|| anyhow!("Please pass CAPI_ASSET_ID"))?;
    log::debug!("Capi asset id: {:?}", str);

    Ok(CapiAssetId(str.parse().map_err(Error::msg)?))
}
