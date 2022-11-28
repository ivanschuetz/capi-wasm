use crate::error::FrError;
use crate::js::bridge::log_wrap_new;
use crate::model::dao_js::DaoJs;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ViewDaoProvider {
    async fn get(&self, pars: ViewDaoParJs) -> Result<ViewDaoResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct ViewDaoParJs {
    pub dao_id: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ViewDaoResJs {
    pub dao: DaoJs,
    pub shares_available: String,
    pub investors_share: String,
    pub available_funds: String,
    pub customer_payment_deeplink: String,
}

#[wasm_bindgen(js_name=viewDao)]
pub async fn view_dao(pars: ViewDaoParJs) -> Result<ViewDaoResJs, FrError> {
    log_wrap_new("view_dao", pars, async move |pars| {
        providers()?.view_dao.get(pars).await
    })
    .await
}
