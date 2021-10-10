use crate::server::api::Api;
use algonaut::{
    algod::{v2::Algod, AlgodBuilder, AlgodCustomEndpointBuilder},
    indexer::{v2::Indexer, IndexerBuilder, IndexerCustomEndpointBuilder},
};

#[derive(Debug)]
pub enum Env {
    #[allow(dead_code)]
    Local,
    #[allow(dead_code)]
    Test,
}

pub fn environment() -> Env {
    #[cfg(feature = "testenv")]
    let env = Env::Test;
    #[cfg(not(feature = "testenv"))]
    let env = Env::Local;

    log::info!("Environment: {:?}", env);
    env
}

pub fn algod(env: &Env) -> Algod {
    match env {
        Env::Local => private_network_algod(),
        Env::Test => testnet_algod("Ii8MvLymlZ8mxE5hT94KG4nEWfH1A7cP6WMWTfkk"),
    }
}

#[allow(dead_code)]
pub fn indexer(env: &Env) -> Indexer {
    match env {
        Env::Local => private_network_indexer(),
        Env::Test => testnet_indexer("Ii8MvLymlZ8mxE5hT94KG4nEWfH1A7cP6WMWTfkk"),
    }
}

pub fn api(env: &Env) -> Api {
    Api::new(
        match env {
            Env::Local => "http://127.0.0.1:3030",
            Env::Test => "http://app.shares.finance:3030",
        }
        .to_owned(),
    )
}

#[allow(dead_code)]
fn private_network_algod() -> Algod {
    AlgodBuilder::new()
        .bind("http://127.0.0.1:53630")
        .auth("44d70009a00561fe340b2584a9f2adc6fec6a16322554d44f56bef9e682844b9")
        .build_v2()
        // expect: build returns an error if the URL or token are not provided or have an invalid format,
        // we are passing verified hardcoded values.
        .expect("Couldn't initialize algod")
}

#[allow(dead_code)]
fn private_network_indexer() -> Indexer {
    IndexerBuilder::new()
        .bind("http://127.0.0.1:8980")
        .build_v2()
        // expect: build returns an error if the URL is not provided or has an invalid format,
        // we are passing a verified hardcoded value.
        .expect("Couldn't initialize indexer")
}

#[allow(dead_code)]
fn testnet_algod(api_key: &str) -> Algod {
    AlgodCustomEndpointBuilder::new()
        .bind("https://testnet-algorand.api.purestake.io/ps2/")
        .headers(vec![("x-api-key", api_key)])
        .build_v2()
        // expect: build returns an error if the URL or token are not provided or have an invalid format,
        // we are passing verified hardcoded values.
        .expect("Couldn't initialize algod")
}

#[allow(dead_code)]
fn testnet_indexer(api_key: &str) -> Indexer {
    IndexerCustomEndpointBuilder::new()
        .bind("https://testnet-algorand.api.purestake.io/idx2/")
        .headers(vec![("x-api-key", api_key)])
        .build_v2()
        // expect: build returns an error if the URL or token are not provided or have an invalid format,
        // we are passing verified hardcoded values.
        .expect("Couldn't initialize algod")
}
