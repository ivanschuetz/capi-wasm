use anyhow::{anyhow, Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;
use web_sys::Storage;

pub fn storage_get_str(key: &str) -> Result<Option<String>> {
    log::debug!("Will retrieve local storage key: {key}");
    let res = storage()?.get_item(key).map_err(to_anyhow)?;
    log::debug!("local storage got string: {res:?} for key: {key}");
    Ok(res)
}

pub fn storage_set_str(key: &str, value: &str) -> Result<()> {
    storage()?.set_item(key, value).map_err(to_anyhow)
}

pub fn storage_set<T>(key: &str, value: &T) -> Result<()>
where
    T: Serialize,
{
    storage_set_str(key, &serde_json::to_string(value)?)
}

pub fn storage_get<T>(key: &str) -> Result<Option<T>>
where
    T: DeserializeOwned,
{
    let str = storage_get_str(key)?;
    Ok(match str {
        Some(s) => Some(serde_json::from_str(&s)?),
        None => None,
    })
}

#[allow(dead_code)] // can be used only for debugging
pub fn storage_clear_all() -> Result<()> {
    log::debug!("Will clear all local storage..");
    storage()?.clear().map_err(to_anyhow)
}

fn storage() -> Result<Storage> {
    web_sys::window()
        .ok_or(anyhow!("Unexpected: no window"))?
        .local_storage()
        .map_err(to_anyhow)?
        .ok_or(anyhow!("Unexpected: no storage"))
}

fn to_anyhow(value: JsValue) -> anyhow::Error {
    Error::msg(format!("{value:?}"))
}
