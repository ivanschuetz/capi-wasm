use crate::service::storage::storage_get_str;
use anyhow::{anyhow, Error, Result};
use mbase::{
    dependencies::{network, DataType, Network},
    models::{
        capi_deps::{CapiAddress, CapiAssetDaoDeps},
        funds::FundsAssetId,
    },
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
    // init_log().unwrap(); // in case it's needed to debug first access - currently logs are initialized about at the same time as first access

    let str = if is_runtime_env()? {
        storage_get_str("FUNDS_ASSET_ID")?
    } else {
        option_env!("FUNDS_ASSET_ID").map(|s| s.to_owned()) // key needs to be a literal here
    }
    .ok_or_else(|| anyhow!("Please pass FUNDS_ASSET_ID"))?;

    log::debug!("Funds asset id: {:?}", str);

    Ok(FundsAssetId(str.parse().map_err(Error::msg)?))
}

pub fn capi_address() -> Result<CapiAddress> {
    // init_log().unwrap(); // in case it's needed to debug first access - currently logs are initialized about at the same time as first access

    let str = if is_runtime_env()? {
        storage_get_str("CAPI_ADDRESS")?
    } else {
        option_env!("CAPI_ADDRESS").map(|s| s.to_owned()) // key needs to be a literal here
    }
    .ok_or_else(|| anyhow!("Please pass CAPI_ADDRESS"))?;

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

// historical: tried to refactor the wasm env variable fetch, but didn't work.
// appears to work by writing a macro, but that seems overcomplicated, for this. for now just duplicating the code
// fn read_wasm_env_variable(key: &str) -> Result<Option<String>> {
//     if is_runtime_env()? {
//         // this doesn't accept non-literals
//         Ok(option_env!(key).map(|s| s.to_owned()))
//         // this doesn't read build time variables
//         // match std::env::var(key) {
//         //     Ok(value) => Ok(Some(value)),
//         //     Err(VarError::NotPresent) => Ok(None),
//         //     Err(e) => Err(e.into()),
//         // }
//     } else {
//         storage_get_str(key)
//     }
// }
// historical: this macro compiles but couldn't find env variable at runtime?
// duplicate if/else is better anyway, it's just in 2 places now
// macro_rules! read_wasm_env_variable {
//     ($key: tt) => {
//         if is_runtime_env()? {
//             storage_get_str($key)
//         } else {
//            Ok(option_env!($key).map(|s| s.to_owned()))
//         }
//     };
// }

/// Whether the WASM file was built to be used by external QA / frontend devs who will use a local network,
/// but don't have access to the WASM source (can't build it)
/// the background of this, is that they have to be able run a program that initializes/resets (create funds asset, funds etc.) the environment,
/// which generates an asset id and capi address, which have to be passed to WASM as build-time env. variables
/// but since they don't have access to the WASM source (we want to limit what we share - it also contains base etc.)
/// they can't pass those values to the build
/// so the values have to be passed at runtime, using the frontend, and read from local storage
/// we use this flag to enable this.
fn is_runtime_env() -> Result<bool> {
    let str = option_env!("RUNTIME_ENV").unwrap_or_else(|| "0");
    log::debug!("RUNTIME_ENV: {:?}", str);

    if str == "1" {
        Ok(true)
    } else if str == "0" {
        Ok(false) // static (build time) env variables
    } else {
        Err(anyhow!("Invalid RUNTIME_ENV value: {str}"))
    }
}
