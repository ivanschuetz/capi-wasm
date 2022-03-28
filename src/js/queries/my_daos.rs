use crate::dependencies::{api, capi_deps};
use crate::js::common::{parse_bridge_pars, to_bridge_res};
use anyhow::{Error, Result};
use core::dependencies::{algod, indexer};
use core::queries::my_daos::{my_daos, MyStoredDao};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_my_daos(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_my_daos, pars: {:?}", pars);
    to_bridge_res(_bridge_my_daos(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_my_daos(pars: MyDaosParJs) -> Result<MyDaosResJs> {
    let algod = algod();
    let api = api();
    let indexer = indexer();
    let capi_deps = capi_deps()?;

    let address = pars.address.parse().map_err(Error::msg)?;

    let daos = my_daos(&algod, &indexer, &address, &api, &capi_deps).await?;

    Ok(MyDaosResJs {
        daos: daos.into_iter().map(|p| p.into()).collect(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct MyDaosParJs {
    pub address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MyDaosResJs {
    pub daos: Vec<MyDaoJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MyDaoJs {
    pub url_rel: String,
    pub name: String,
    pub created_by_me: String,
    pub invested_by_me: String,
}

impl From<MyStoredDao> for MyDaoJs {
    fn from(p: MyStoredDao) -> Self {
        MyDaoJs {
            url_rel: format!("/{}", p.dao.id().to_string()),
            name: p.dao.specs.name,
            created_by_me: p.created_by_me.to_string(),
            invested_by_me: p.invested_by_me.to_string(),
        }
    }
}
