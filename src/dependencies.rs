use core::dependencies::{network, Network};

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
