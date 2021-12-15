use crate::js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs};
use algonaut::algod::v2::Algod;
use anyhow::{anyhow, Result};
use core::network_util::wait_for_pending_transaction;

pub async fn submit_apps_optins_from_js(algod: &Algod, optins: &[SignedTxFromJs]) -> Result<()> {
    if optins.len() != 1 {
        return Err(anyhow!("Invalid app optins count: {}", optins.len()));
    }
    let central_optin = &optins[0];
    submit_apps_optins(&algod, central_optin).await
}

async fn submit_apps_optins(algod: &Algod, central_optin: &SignedTxFromJs) -> Result<()> {
    let app_optins_txs = vec![signed_js_tx_to_signed_tx1(central_optin)?];
    let res = algod.broadcast_signed_transactions(&app_optins_txs).await?;
    let _ = wait_for_pending_transaction(&algod, &res.tx_id);

    Ok(())
}
