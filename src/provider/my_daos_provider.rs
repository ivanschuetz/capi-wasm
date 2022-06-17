use anyhow::Result;
use async_trait::async_trait;
use base::queries::my_daos::MyStoredDao;
use serde::{Deserialize, Serialize};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait MyDaosProvider {
    async fn get(&self, pars: MyDaosParJs) -> Result<MyDaosResJs>;
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
            name: p.dao.name,
            created_by_me: p.created_by_me.to_string(),
            invested_by_me: p.invested_by_me.to_string(),
        }
    }
}
