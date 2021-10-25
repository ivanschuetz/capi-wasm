use algonaut::{
    algod::v2::Algod,
    core::{Address, MicroAlgos},
    model::algod::v2::TealKeyValue,
};
use anyhow::{anyhow, Result};
use data_encoding::BASE64;

// TODO refactor with central_app_state in core proj

pub async fn central_received_total(algod: &Algod, app_id: u64) -> Result<MicroAlgos> {
    Ok(algod
        .application_information(app_id)
        .await?
        .params
        .global_state
        .iter()
        .find(|kv| kv.key == BASE64.encode(b"CentralReceivedTotal").to_owned())
        // .ok_or({
        //     anyhow!("Invalid app state: CentralReceivedTotal isn't set. App: {}. Please contact support <TODO>", pars.app_id)})?
        // TODO confirm that not existent global state means 0
        .map(|kv| MicroAlgos(kv.value.uint))
        .unwrap_or_else(|| MicroAlgos(0)))
}

pub async fn harvested_total_from_local_vars(
    user_local_vars: &Vec<TealKeyValue>,
) -> Result<MicroAlgos> {
    Ok(user_local_vars
        .iter()
        .find(|kv| kv.key == BASE64.encode(b"HarvestedTotal").to_owned())
        .map(|kv| MicroAlgos(kv.value.uint))
        // TODO confirm that not existent local state key means 0
        // we currently assume it's the case
        .unwrap_or_else(|| MicroAlgos(0)))
}

pub async fn owned_shares_count_from_local_vars(
    user_local_vars: &Vec<TealKeyValue>,
) -> Result<u64> {
    Ok(user_local_vars
        .iter()
        .find(|kv| kv.key == BASE64.encode(b"Shares").to_owned())
        .map(|kv| kv.value.uint)
        // TODO confirm that not existent local state key means 0
        // we currently assume it's the case
        .unwrap_or_else(|| 0))
}

// WARNING: not generic, only for investor (see HACK in body)
pub async fn local_vars(
    algod: &Algod,
    address: &Address,
    app_id: u64,
) -> Result<Vec<TealKeyValue>> {
    let investor_account_infos = algod.account_information(address).await?;

    Ok(investor_account_infos
        .apps_local_state
        .into_iter()
        .find(|ls| ls.id == app_id)
        // HACK: we're matching against this text in JS
        // TODO the result should be either an enum (invested/not invested) or error have status codes
        // TODO also, confirm that no local state unequivocally means that user isn't invested
        // .ok_or(anyhow!("No local state for app: {}.", pars.app_id))?
        .ok_or(anyhow!("You're not invested in this project.",))?
        .key_value)
}

//////////
/// TODO (low prio) consider removing
//////////

// convenience - maybe used in the future
#[allow(dead_code)]
pub async fn owned_shares_count(
    algod: &Algod,
    address: &Address,
    central_app_id: u64,
) -> Result<u64> {
    let local_vars = local_vars(algod, address, central_app_id).await?;
    owned_shares_count_from_local_vars(&local_vars).await
}

// convenience - maybe used in the future
#[allow(dead_code)]
pub async fn harvested_total(
    algod: &Algod,
    address: &Address,
    central_app_id: u64,
) -> Result<MicroAlgos> {
    let local_vars = local_vars(algod, address, central_app_id).await?;
    harvested_total_from_local_vars(&local_vars).await
}
