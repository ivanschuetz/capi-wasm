use algonaut::transaction::{SignedTransaction, Transaction};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::to_value;
use std::fmt::{Debug, Display};
use tsify::Tsify;
use wasm_bindgen::JsValue;

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SignedTxFromJs {
    pub blob: Vec<u8>,
}

pub fn to_my_algo_tx(tx: &Transaction) -> Result<Value, JsValue> {
    my_algo::to_my_algo_transaction::to_my_algo_transaction_value(tx).map_err(to_js_value)
}

// TODO remove the other one, use this (js "decorator" refactoring)
pub fn to_my_algo_tx1(tx: &Transaction) -> Result<Value> {
    my_algo::to_my_algo_transaction::to_my_algo_transaction_value(tx)
}

pub fn to_my_algo_txs(txs: &[Transaction]) -> Result<Vec<Value>, JsValue> {
    let mut res = vec![];
    for tx in txs {
        res.push(to_my_algo_tx(tx)?);
    }
    Ok(res)
}

// TODO remove the other one, use this (js "decorator" refactoring)
pub fn to_my_algo_txs1(txs: &[Transaction]) -> Result<Vec<Value>> {
    let mut res = vec![];
    for tx in txs {
        res.push(to_my_algo_tx1(tx)?);
    }
    Ok(res)
}

pub fn signed_js_tx_to_signed_tx(
    signed_js_tx: &SignedTxFromJs,
) -> Result<SignedTransaction, JsValue> {
    rmp_serde::from_slice(&signed_js_tx.blob).map_err(to_js_value)
}

// TODO remove the other one, use this (js "decorator" refactoring)
pub fn signed_js_tx_to_signed_tx1(signed_js_tx: &SignedTxFromJs) -> Result<SignedTransaction> {
    Ok(rmp_serde::from_slice(&signed_js_tx.blob)?)
}

pub fn signed_js_txs_to_signed_tx(
    txs: &[SignedTxFromJs],
) -> Result<Vec<SignedTransaction>, JsValue> {
    let mut res = vec![];
    for tx in txs {
        res.push(signed_js_tx_to_signed_tx(tx)?);
    }
    Ok(res)
}

pub fn signed_js_txs_to_signed_tx1(txs: &[SignedTxFromJs]) -> Result<Vec<SignedTransaction>> {
    let mut res = vec![];
    for tx in txs {
        res.push(signed_js_tx_to_signed_tx1(tx)?);
    }
    Ok(res)
}

pub fn to_js_value<T: Display>(t: T) -> JsValue {
    JsValue::from_str(&format!("{}", t))
}

pub fn to_js_res<T: Serialize>(res: T) -> Result<JsValue, JsValue> {
    to_value(&res).map_err(to_js_value)
}
