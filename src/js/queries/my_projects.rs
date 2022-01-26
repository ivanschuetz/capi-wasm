use crate::js::common::{parse_bridge_pars, to_bridge_res};
use crate::teal::programs;
use anyhow::{Error, Result};
use core::dependencies::{algod, indexer};
use core::queries::my_projects::{my_projects, MyStoredProject};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_my_projects(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_my_projects, pars: {:?}", pars);
    to_bridge_res(_bridge_my_projects(parse_bridge_pars(pars)?).await)
}

pub async fn _bridge_my_projects(pars: MyProjectsParJs) -> Result<MyProjectsResJs> {
    let algod = algod();
    let indexer = indexer();

    let address = pars.address.parse().map_err(Error::msg)?;

    let projects = my_projects(&algod, &indexer, &address, &programs().escrows).await?;

    Ok(MyProjectsResJs {
        projects: projects.into_iter().map(|p| p.into()).collect(),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct MyProjectsParJs {
    pub address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MyProjectsResJs {
    pub projects: Vec<MyProjectJs>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MyProjectJs {
    pub url_rel: String,
    pub name: String,
    pub created_by_me: String,
    pub invested_by_me: String,
}

impl From<MyStoredProject> for MyProjectJs {
    fn from(p: MyStoredProject) -> Self {
        MyProjectJs {
            url_rel: format!("/{}", p.project.id.to_string()),
            name: p.project.project.specs.name,
            created_by_me: p.created_by_me.to_string(),
            invested_by_me: p.invested_by_me.to_string(),
        }
    }
}
