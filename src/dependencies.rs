use core::dependencies::{env, Env};

use crate::server::api::Api;

/// Convenience to not have to pass env everywhere
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
