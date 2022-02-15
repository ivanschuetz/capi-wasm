use core::{
    dependencies::{network, Network},
    funds::FundsAssetId,
};

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

pub fn funds_asset_specs() -> FundsAssetSpecs {
    funds_asset_specs_for_net(&network())
}

pub fn funds_asset_specs_for_net(net: &Network) -> FundsAssetSpecs {
    match net {
        // For private network, we insert funds asset id manually (run a core test, get from logs) for now.
        // The id always changes, as on  tests (or manually calling the reset script) the network and assets are re-created.
        // We could retrieve it from one of the test accounts (after a test they should be opted in and funded)
        // but then function needs to be async / converted to sync.. (later case might need additional deps / also check WASM)
        // might do later.
        Network::Private | Network::SandboxPrivate => FundsAssetSpecs {
            id: FundsAssetId(6),
            decimals: 6,
        },
        // USDC (testnet)
        Network::Test => FundsAssetSpecs {
            id: FundsAssetId(10458941),
            decimals: 6,
        },
    }
}

/// This is WASM-only as the decimals are needed only for formatting - we don't need this in core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FundsAssetSpecs {
    pub id: FundsAssetId,
    // Technically (algonaut) decimals is u64, but Decimal wants u32 and realistically u32 is enough. + we currently hardcode this.
    pub decimals: u32,
}
