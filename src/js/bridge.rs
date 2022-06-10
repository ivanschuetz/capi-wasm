use crate::{
    js::common::{parse_bridge_pars, to_bridge_res, to_js_res},
    provider::providers,
};
use anyhow::Result;
use std::future::Future;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_create_dao_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_create_dao_assets", pars, async move |pars| {
        providers()?
            .create_assets
            .txs(parse_bridge_pars(pars)?)
            .await
            .map_err(|e| e.into())
            .and_then(|r| to_js_res(&r))
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_create_dao", pars, async move |pars| {
        providers()?
            .create_dao
            .txs(parse_bridge_pars(pars)?)
            .await
            .map_err(|e| e.into())
            .and_then(|r| to_js_res(&r))
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_create_dao", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .create_dao
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_load_funds_activity(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_load_funds_activity", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .funds_activity
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_balance(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_balance", pars, async move |pars| {
        to_bridge_res(providers()?.balance.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_buy_shares", pars, async move |pars| {
        providers()?
            .buy_shares
            .txs(parse_bridge_pars(pars)?)
            .await
            .map_err(|e| e.into())
            .and_then(|r| to_js_res(&r))
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_get_user_shares_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_get_user_shares_count", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .shares_count
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_load_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_load_dao", pars, async move |pars| {
        to_bridge_res(providers()?.dao.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_opt_in_to_apps_if_needed(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_opt_in_to_apps_if_needed", pars, async move |pars| {
        to_bridge_res(providers()?.app_optin.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_buy_shares", pars, async move |pars| {
        providers()?
            .buy_shares
            .submit(parse_bridge_pars(pars)?)
            .await
            .map_err(|e| e.into())
            .and_then(|r| to_js_res(&r))
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_claim", pars, async move |pars| {
        to_bridge_res(providers()?.claim.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_load_investment(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_load_investment", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .investment
                .get_investor_data(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_load_available_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_load_available_shares", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .investment
                .available_shares(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_claim", pars, async move |pars| {
        to_bridge_res(providers()?.claim.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_lock", pars, async move |pars| {
        to_bridge_res(providers()?.lock.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_lock", pars, async move |pars| {
        to_bridge_res(providers()?.lock.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_pay_dao", pars, async move |pars| {
        to_bridge_res(providers()?.pay_dao.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_pay_dao", pars, async move |pars| {
        to_bridge_res(providers()?.pay_dao.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_holders_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_holders_count", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .holders_count
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_income_vs_spending(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_income_vs_spending", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .income_vs_spending
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_my_daos(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_my_daos", pars, async move |pars| {
        to_bridge_res(providers()?.my_daos.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_my_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_my_shares", pars, async move |pars| {
        to_bridge_res(providers()?.my_shares.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_shares_distribution(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_shares_distribution", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .shares_distribution
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_load_roadmap(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_load_roadmap", pars, async move |pars| {
        to_bridge_res(providers()?.roadmap.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_add_roadmap_item", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .add_roadmap_item
                .txs(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_add_roadmap_item", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .add_roadmap_item
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_unlock", pars, async move |pars| {
        to_bridge_res(providers()?.unlock.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_unlock", pars, async move |pars| {
        to_bridge_res(providers()?.unlock.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_check_for_updates(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_check_for_updates", pars, async move |pars| {
        to_bridge_res(providers()?.app_updates.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_update_app_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_update_app_txs", pars, async move |pars| {
        to_bridge_res(providers()?.update_app.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_update_app(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_update_app", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .update_app
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

/// To pre fill the form to update data
#[wasm_bindgen]
pub async fn bridge_updatable_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_updatable_data", pars, async move |pars| {
        to_bridge_res(providers()?.update_data.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_update_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_update_data", pars, async move |pars| {
        to_bridge_res(providers()?.update_data.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_update_dao_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_update_dao_data", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .update_data
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_drain", pars, async move |pars| {
        to_bridge_res(providers()?.drain.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_drain", pars, async move |pars| {
        to_bridge_res(providers()?.drain.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_view_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_view_dao", pars, async move |pars| {
        to_bridge_res(providers()?.view_dao.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_withdraw", pars, async move |pars| {
        to_bridge_res(providers()?.withdraw.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_withdraw", pars, async move |pars| {
        to_bridge_res(providers()?.withdraw.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_load_withdrawals(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_load_withdrawals", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .withdrawals_history
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_calculate_shares_price(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_calculate_shares_price", pars, async move |pars| {
        providers()?
            .calculate_total_price
            .get(parse_bridge_pars(pars)?)
            .await
            .map_err(|e| e.into())
            .and_then(|r| to_js_res(&r))
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_my_dividend(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_my_dividend", pars, async move |pars| {
        to_bridge_res(providers()?.dividend.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn get_balance_change(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("get_balance_change", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .balance
                .get_balance_change(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_reclaim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_reclaim", pars, async move |pars| {
        to_bridge_res(providers()?.reclaim.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_submit_reclaim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_submit_reclaim", pars, async move |pars| {
        to_bridge_res(providers()?.reclaim.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_description(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_description", pars, async move |pars| {
        to_bridge_res(providers()?.description.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn bridge_reserve_wyre(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("bridge_reserve_wyre", pars, async move |pars| {
        to_bridge_res(providers()?.wyre.reserve(parse_bridge_pars(pars)?).await)
    })
    .await
}

async fn log_wrap<Fut>(
    label: &str,
    pars: JsValue,
    handler: impl FnOnce(JsValue) -> Fut + Send,
) -> Result<JsValue, JsValue>
where
    Fut: Future<Output = Result<JsValue, JsValue>>,
{
    log::debug!("{label}, pars: {:?}", pars);
    let res = handler(pars.clone()).await;
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}, pars: {pars:?}");
    }
    res
}
