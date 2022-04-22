use crate::{
    js::common::{parse_bridge_pars, to_bridge_res},
    provider::providers,
};
use anyhow::Result;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_create_dao_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_dao_assets, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .create_assets
            .txs(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_create_dao, pars: {:?}", pars);
    to_bridge_res(providers()?.create_dao.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_create_dao, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .create_dao
            .submit(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_load_funds_activity(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_funds_activity, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .funds_activity
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_balance(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_balance, pars: {:?}", pars);
    to_bridge_res(providers()?.balance.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_buy_shares, pars: {:?}", pars);
    to_bridge_res(providers()?.buy_shares.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_get_user_shares_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_get_user_shares_count, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .shares_count
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_load_dao_user_view(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_dao_user_view, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .dao_user_view
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_opt_in_to_apps_if_needed(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_opt_in_to_apps_if_needed, pars: {:?}", pars);
    to_bridge_res(providers()?.app_optin.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_buy_shares, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .buy_shares
            .submit(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_claim, pars: {:?}", pars);
    to_bridge_res(providers()?.claim.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_load_investment(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_investment, pars: {:?}", pars);
    to_bridge_res(providers()?.investment.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_claim, pars: {:?}", pars);
    to_bridge_res(providers()?.claim.submit(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_lock, pars: {:?}", pars);
    to_bridge_res(providers()?.lock.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_lock, pars: {:?}", pars);
    to_bridge_res(providers()?.lock.submit(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_pay_dao, pars: {:?}", pars);
    to_bridge_res(providers()?.pay_dao.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_pay_dao, pars: {:?}", pars);
    to_bridge_res(providers()?.pay_dao.submit(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_holders_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_holders_count, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .holders_count
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_income_vs_spending(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_income_vs_spending, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .income_vs_spending
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_my_daos(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_my_daos, pars: {:?}", pars);
    to_bridge_res(providers()?.my_daos.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_my_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_my_shares, pars: {:?}", pars);
    to_bridge_res(providers()?.my_shares.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_shares_distribution(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_shares_distribution, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .shares_distribution
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_load_roadmap(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_roadmap, pars: {:?}", pars);
    to_bridge_res(providers()?.roadmap.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_add_roadmap_item, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .add_roadmap_item
            .txs(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_submit_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_add_roadmap_item, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .add_roadmap_item
            .submit(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_unlock, pars: {:?}", pars);
    to_bridge_res(providers()?.unlock.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_unlock, pars: {:?}", pars);
    to_bridge_res(providers()?.unlock.submit(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_check_for_updates(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_check_for_updates, pars: {:?}", pars);
    to_bridge_res(providers()?.app_updates.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_update_app_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_update_app_txs, pars: {:?}", pars);
    to_bridge_res(providers()?.update_app.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_update_app(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_update_app, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .update_app
            .submit(parse_bridge_pars(pars)?)
            .await,
    )
}

/// To pre fill the form to update data
#[wasm_bindgen]
pub async fn bridge_updatable_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_updatable_data, pars: {:?}", pars);
    to_bridge_res(providers()?.update_data.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_update_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_update_data, pars: {:?}", pars);
    to_bridge_res(providers()?.update_data.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_update_dao_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_update_dao_data, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .update_data
            .submit(parse_bridge_pars(pars)?)
            .await,
    )
}

#[wasm_bindgen]
pub async fn bridge_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_drain, pars: {:?}", pars);
    to_bridge_res(providers()?.drain.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_drain, pars: {:?}", pars);
    to_bridge_res(providers()?.drain.submit(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_view_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_view_dao, pars: {:?}", pars);
    to_bridge_res(providers()?.view_dao.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_load_dao_user_view_with_id(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_dao_user_view_with_id, pars: {:?}", pars);
    to_bridge_res(providers()?.dao_with_id.get(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_withdraw, pars: {:?}", pars);
    to_bridge_res(providers()?.withdraw.txs(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_submit_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_submit_withdraw, pars: {:?}", pars);
    to_bridge_res(providers()?.withdraw.submit(parse_bridge_pars(pars)?).await)
}

#[wasm_bindgen]
pub async fn bridge_load_withdrawals(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_withdrawals, pars: {:?}", pars);
    to_bridge_res(
        providers()?
            .withdrawals_history
            .get(parse_bridge_pars(pars)?)
            .await,
    )
}
