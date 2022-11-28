use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, common::SignedTxFromJs, to_sign_js::ToSignJs},
};

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait PayDaoProvider {
    async fn txs(&self, pars: PayDaoParJs) -> Result<PayDaoResJs, FrError>;
    async fn submit(&self, pars: SubmitPayDaoParJs) -> Result<SubmitPayDaoResJs, FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct PayDaoParJs {
    pub customer_address: String,
    pub dao_id: String,
    pub amount: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct PayDaoResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitPayDaoParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SubmitPayDaoResJs {}

#[wasm_bindgen(js_name=payDao)]
pub async fn pay_dao(pars: PayDaoParJs) -> Result<PayDaoResJs, FrError> {
    log_wrap_new("pay_dao", pars, async move |pars| {
        providers()?.pay_dao.txs(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitPayDao)]
pub async fn submit_pay_dao(pars: SubmitPayDaoParJs) -> Result<SubmitPayDaoResJs, FrError> {
    log_wrap_new("submit_pay_dao", pars, async move |pars| {
        providers()?.pay_dao.submit(pars).await
    })
    .await
}
