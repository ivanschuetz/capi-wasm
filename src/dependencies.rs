use anyhow::{anyhow, Error, Result};
use base::capi_deps::{CapiAddress, CapiAssetDaoDeps};
use mbase::{
    dependencies::{network, DataType, Network},
    models::funds::FundsAssetId,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{convert::TryInto, str::FromStr};

/// URL determined by environment variable
pub fn explorer_base_url<'a>() -> &'a str {
    explorer_base_url_for_net(&network())
}

pub fn explorer_base_url_for_net<'a>(net: &Network) -> &'a str {
    match net {
        // No explorer for private network - we just test that it opens and searches
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
        escrow_percentage: Decimal::from_str("0.01")?.try_into()?,
        address: capi_address()?,
    })
}

/// This is WASM-only as the decimals are needed only for formatting - we don't need this in core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

pub fn capi_address() -> Result<CapiAddress> {
    let str = option_env!("CAPI_ADDRESS").ok_or_else(|| anyhow!("Please pass CAPI_ADDRESS"))?;
    log::debug!("Capi address: {:?}", str);

    Ok(CapiAddress(str.parse().map_err(Error::msg)?))
}

pub fn data_type() -> Result<DataType> {
    let str = option_env!("DATA_TYPE").ok_or_else(|| anyhow!("Please pass DATA_TYPE"))?;
    log::debug!("Data type: {:?}", str);

    match str {
        "real" => Ok(DataType::Real),
        "mock" => Ok(DataType::Mock),
        _ => Err(anyhow!("Invalid data type: {str}")),
    }
}
