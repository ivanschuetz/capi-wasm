use core::dependencies::{env, network, Env, Network};

use crate::server::api::Api;

/// URL determined by environment variable
pub fn api() -> Api {
    api_for_env(&env())
}

fn api_for_env(env: &Env) -> Api {
    Api::new(
        match env {
            Env::Local => "http://127.0.0.1:3030",
            Env::Test => "http://test.app.capi.finance:3030",
        }
        .to_owned(),
    )
}

/// URL determined by environment variable
pub fn explorer_base_url<'a>() -> &'a str {
    explorer_base_url_for_net(&network())
}

pub fn explorer_base_url_for_net<'a>(net: &Network) -> &'a str {
    match net {
        // We don't have an explorer for private network - this will not find anything - so we just test that it opens the explorer and searches
        Network::Private | Network::SandboxPrivate => "https://testnet.algoexplorer.io/",
    }
}
