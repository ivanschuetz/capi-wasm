use crate::{
    error::FrError,
    provider::my_daos_provider::{MyDaoJs, MyDaosParJs, MyDaosProvider, MyDaosResJs},
};
use anyhow::Result;
use async_trait::async_trait;

use super::req_delay;

pub struct MyDaosProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl MyDaosProvider for MyDaosProviderMock {
    async fn get(&self, _: MyDaosParJs) -> Result<MyDaosResJs, FrError> {
        req_delay().await;

        Ok(MyDaosResJs {
            daos: vec![
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 1".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 2".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: None,
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Lorem ipsum dolor sit amet, consectetur".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "I created this one".to_owned(),
                    created_by_me: "true".to_owned(),
                    invested_by_me: "false".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "I created this one too".to_owned(),
                    created_by_me: "true".to_owned(),
                    invested_by_me: "false".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 3".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "I created *and invested* in this one".to_owned(),
                    created_by_me: "true".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 5".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 6".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 7".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 8".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 9".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 10".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 11".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 12".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: None,
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 13".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: None,
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 14".to_owned(),
                    created_by_me: "true".to_owned(),
                    invested_by_me: "false".to_owned(),
                    image_url: None,
                },
                MyDaoJs {
                    url_rel: "/123".to_owned(),
                    name: "Cool business 15".to_owned(),
                    created_by_me: "false".to_owned(),
                    invested_by_me: "true".to_owned(),
                    image_url: Some("https://placekitten.com/1033/360".to_owned()),
                },
            ],
        })
    }
}
