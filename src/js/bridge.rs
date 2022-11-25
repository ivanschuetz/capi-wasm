use crate::{
    error::FrError,
    js::common::{parse_bridge_pars, to_bridge_res, to_js_res},
    provider::providers,
};
use anyhow::Result;
use serde::Serialize;
use std::{convert::TryInto, future::Future};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=createDaoAssetsTxs)]
pub async fn create_dao_assets_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("create_dao_assets", pars, async move |pars| {
        to_js(
            providers()?
                .create_assets
                .txs(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=createDao)]
pub async fn create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("create_dao", pars, async move |pars| {
        to_js(providers()?.create_dao.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitCreateDao)]
pub async fn submit_create_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_create_dao", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .create_dao
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=loadFundsActivity)]
pub async fn load_funds_activity(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("load_funds_activity", pars, async move |pars| {
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
pub async fn balance(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("balance", pars, async move |pars| {
        to_bridge_res(providers()?.balance.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=buyShares)]
pub async fn buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("buy_shares", pars, async move |pars| {
        providers()?
            .buy_shares
            .txs(parse_bridge_pars(pars)?)
            .await
            .map_err(|e| e.into())
            .and_then(|r| to_js_res(&r))
    })
    .await
}

#[wasm_bindgen(js_name=getUserSharesCount)]
pub async fn get_user_shares_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("get_user_shares_count", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .shares_count
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=loadDao)]
pub async fn load_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("load_dao", pars, async move |pars| {
        to_bridge_res(providers()?.dao.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=optInToAppsIfNeeded)]
pub async fn opt_in_to_apps_if_needed(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("opt_in_to_apps_if_needed", pars, async move |pars| {
        to_bridge_res(providers()?.app_optin.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitBuyShares)]
pub async fn submit_buy_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_buy_shares", pars, async move |pars| {
        to_js(
            providers()?
                .buy_shares
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen]
pub async fn claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("claim", pars, async move |pars| {
        to_bridge_res(providers()?.claim.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=loadInvestment)]
pub async fn load_investment(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("load_investment", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .investment
                .get_investor_data(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=loadAvailableShares)]
pub async fn load_available_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("load_available_shares", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .investment
                .available_shares(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=submitClaim)]
pub async fn submit_claim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_claim", pars, async move |pars| {
        to_bridge_res(providers()?.claim.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("lock", pars, async move |pars| {
        to_js(providers()?.lock.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitLock)]
pub async fn submit_lock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_lock", pars, async move |pars| {
        to_bridge_res(providers()?.lock.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=payDao)]
pub async fn pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("pay_dao", pars, async move |pars| {
        to_bridge_res(providers()?.pay_dao.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitPayDao)]
pub async fn submit_pay_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_pay_dao", pars, async move |pars| {
        to_bridge_res(providers()?.pay_dao.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=holdersCount)]
pub async fn holders_count(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("holders_count", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .holders_count
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=incomeVsSpending)]
pub async fn income_vs_spending(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("income_vs_spending", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .income_vs_spending
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=myDaos)]
pub async fn my_daos(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("my_daos", pars, async move |pars| {
        to_bridge_res(providers()?.my_daos.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=myShares)]
pub async fn my_shares(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("my_shares", pars, async move |pars| {
        to_bridge_res(providers()?.my_shares.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=sharesDistribution)]
pub async fn shares_distribution(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("shares_distribution", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .shares_distribution
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=loadRoadmap)]
pub async fn load_roadmap(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("load_roadmap", pars, async move |pars| {
        to_bridge_res(providers()?.roadmap.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=addRoadmapItem)]
pub async fn add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("add_roadmap_item", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .add_roadmap_item
                .txs(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=submitAddRoadmapItem)]
pub async fn submit_add_roadmap_item(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_add_roadmap_item", pars, async move |pars| {
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
pub async fn unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("unlock", pars, async move |pars| {
        to_bridge_res(providers()?.unlock.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitUnlock)]
pub async fn submit_unlock(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_unlock", pars, async move |pars| {
        to_bridge_res(providers()?.unlock.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=checkForUpdates)]
pub async fn check_for_updates(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("check_for_updates", pars, async move |pars| {
        to_bridge_res(providers()?.app_updates.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=updateAppTxs)]
pub async fn update_app_txs(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("update_app_txs", pars, async move |pars| {
        to_bridge_res(providers()?.update_app.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitUpdateApp)]
pub async fn submit_update_app(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_update_app", pars, async move |pars| {
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
#[wasm_bindgen(js_name=updatableData)]
pub async fn updatable_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("updatable_data", pars, async move |pars| {
        to_bridge_res(providers()?.update_data.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=updateData)]
pub async fn update_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("update_data", pars, async move |pars| {
        to_js(providers()?.update_data.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitUpdateDaoData)]
pub async fn submit_update_dao_data(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_update_dao_data", pars, async move |pars| {
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
pub async fn drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("drain", pars, async move |pars| {
        to_bridge_res(providers()?.drain.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitDrain)]
pub async fn submit_drain(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_drain", pars, async move |pars| {
        to_bridge_res(providers()?.drain.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=viewDao)]
pub async fn view_dao(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("view_dao", pars, async move |pars| {
        to_bridge_res(providers()?.view_dao.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("withdraw", pars, async move |pars| {
        to_bridge_res(providers()?.withdraw.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitWithdraw)]
pub async fn submit_withdraw(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_withdraw", pars, async move |pars| {
        to_bridge_res(providers()?.withdraw.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=loadWithdrawals)]
pub async fn load_withdrawals(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("load_withdrawals", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .withdrawals_history
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=calculateSharesPrice)]
pub async fn calculate_shares_price(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("calculate_shares_price", pars, async move |pars| {
        to_js(
            providers()?
                .calculate_total_price
                .get(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=calculateMaxFunds)]
pub async fn calculate_max_funds(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("calculate_max_funds", pars, async move |pars| {
        to_js(
            providers()?
                .calculate_total_price
                .max_funds(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=myDividend)]
pub async fn my_dividend(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("my_dividend", pars, async move |pars| {
        to_bridge_res(providers()?.dividend.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=getBalanceChange)]
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
pub async fn reclaim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("reclaim", pars, async move |pars| {
        to_bridge_res(providers()?.reclaim.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitReclaim)]
pub async fn submit_reclaim(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_reclaim", pars, async move |pars| {
        to_bridge_res(providers()?.reclaim.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen]
pub async fn description(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("description", pars, async move |pars| {
        to_bridge_res(providers()?.description.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=reserveWyre)]
pub async fn reserve_wyre(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("reserve_wyre", pars, async move |pars| {
        to_bridge_res(providers()?.wyre.reserve(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=holdersChange)]
pub async fn holders_change(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("holders_change", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .holders_count
                .change(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=rekeyOwner)]
pub async fn rekey_owner(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("rekey_owner", pars, async move |pars| {
        to_js(providers()?.rekey.txs(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitRekeyOwner)]
pub async fn submit_rekey_owner(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_rekey_owner", pars, async move |pars| {
        to_bridge_res(providers()?.rekey.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=raisedFunds)]
pub async fn raised_funds(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("raised_funds", pars, async move |pars| {
        to_js(providers()?.raised.data(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=calculateHash)]
pub async fn calculate_hash(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("calculate_hash", pars, async move |pars| {
        to_js(providers()?.hash.hash(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=wasmVersion)]
pub async fn wasm_version() -> Result<JsValue, JsValue> {
    log_wrap_sync_no_pars("wasm_version", move || {
        to_js(providers()?.metadata.wasm_version())
    })
    .await
}

#[wasm_bindgen(js_name=setDevSettings)]
pub async fn set_dev_settings(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("set_dev_settings", pars, async move |pars| {
        to_js(
            providers()?
                .dev_settings
                .txs(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=submitSetDevSettings)]
pub async fn submit_set_dev_settings(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_set_dev_settings", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .dev_settings
                .submit(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=getTeam)]
pub async fn get_team(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("get_team", pars, async move |pars| {
        to_bridge_res(providers()?.team.get(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=addTeamMember)]
pub async fn add_team_member(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("add_team_member", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .team
                .add_team_member(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=editTeamMember)]
pub async fn edit_team_member(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("edit_team_member", pars, async move |pars| {
        to_bridge_res(
            providers()?
                .team
                .edit_team_member(parse_bridge_pars(pars)?)
                .await,
        )
    })
    .await
}

#[wasm_bindgen(js_name=setTeam)]
pub async fn set_team(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("set_team", pars, async move |pars| {
        to_bridge_res(providers()?.team.set(parse_bridge_pars(pars)?).await)
    })
    .await
}

#[wasm_bindgen(js_name=submitSetTeam)]
pub async fn submit_set_team(pars: JsValue) -> Result<JsValue, JsValue> {
    log_wrap("submit_set_team", pars, async move |pars| {
        to_bridge_res(providers()?.team.submit(parse_bridge_pars(pars)?).await)
    })
    .await
}

fn to_js<T>(res: Result<T, FrError>) -> Result<JsValue, JsValue>
where
    T: Serialize,
{
    match res {
        Ok(r) => to_js_res(r),
        Err(e) => Err(e.try_into()?),
    }
}

/// Wrap function call with logging
/// Used for async functions with parameters
async fn log_wrap<Fut, T>(
    label: &str,
    pars: JsValue,
    handler: impl FnOnce(JsValue) -> Fut + Send,
) -> Result<T, JsValue>
where
    Fut: Future<Output = Result<T, JsValue>>,
{
    log::debug!("{label}, pars: {:?}", pars);
    let res = handler(pars.clone()).await;
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}, pars: {pars:?}");
    }
    res
}

/// Wrap function call with logging
/// Used for async functions without parameters
/// (dead code: might be used in the future)
#[allow(dead_code)]
async fn log_wrap_no_pars<Fut>(
    label: &str,
    handler: impl FnOnce() -> Fut + Send,
) -> Result<JsValue, JsValue>
where
    Fut: Future<Output = Result<JsValue, JsValue>>,
{
    log::debug!("{label}");
    let res = handler().await;
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}");
    }
    res
}

/// Wrap function call with logging
/// Used for sync functions without parameters
async fn log_wrap_sync_no_pars(
    label: &str,
    handler: impl FnOnce() -> Result<JsValue, JsValue> + Send,
) -> Result<JsValue, JsValue> {
    log::debug!("{label}");
    let res = handler();
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}");
    }
    res
}

#[allow(dead_code)]
// TODO make providers() and parse_bridge_pars() return FrError instead of JsError
// then replace log_wrap with this, simplifying the closures to return Result<T, FrError> instead of Result<JsError, JsError>
// (where possible - some providers don't return FrErrors yet, but the idea is that eventually everything does)
async fn wrap<Fut, T>(
    label: &str,
    pars: JsValue,
    handler: impl FnOnce(JsValue) -> Fut + Send,
) -> Result<JsValue, JsValue>
where
    Fut: Future<Output = Result<T, FrError>>,
    T: Serialize,
{
    log::debug!("{label}, pars: {:?}", pars);
    let res = handler(pars.clone()).await;
    if let Err(e) = res.as_ref() {
        log::error!("Error calling {label}: {e:?}, pars: {pars:?}");
    }
    to_js(res)
}
