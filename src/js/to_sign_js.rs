use super::common::to_my_algo_tx1;
use crate::service::wallet_connect_tx::WalletConnectTx;
use algonaut::transaction::Transaction;
use anyhow::{Error, Result};
use serde::Serialize;
use serde_json::Value;
use tsify::Tsify;

// We always return transactions serialized to both my algo and wallet connect formats
// this could be optimized by passing the connected wallet type as parameter - so we return just the format for it
// but that seems overkill, for now at least
#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ToSignJs {
    pub my_algo: Vec<Value>,
    pub wc: Vec<WalletConnectTx>,
}

impl ToSignJs {
    pub fn new(txs: Vec<Transaction>) -> Result<ToSignJs> {
        let mut my_algo_txs = vec![];
        let mut wc_txs = vec![];

        for tx in txs {
            my_algo_txs.push(to_my_algo_tx1(&tx).map_err(Error::msg)?);
            wc_txs.push(WalletConnectTx::new(&tx, "")?);
        }

        Ok(ToSignJs {
            my_algo: my_algo_txs,
            wc: wc_txs,
        })
    }
}
