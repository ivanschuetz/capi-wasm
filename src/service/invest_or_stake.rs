use crate::js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs};
use algonaut::algod::v2::Algod;
use anyhow::{anyhow, Result};
use make::network_util::wait_for_pending_transaction;

use super::constants::WITHDRAWAL_SLOT_COUNT;

pub async fn submit_apps_optins_from_js(algod: &Algod, optins: &[SignedTxFromJs]) -> Result<()> {
    if optins.len() != 1 + WITHDRAWAL_SLOT_COUNT as usize {
        return Err(anyhow!("Invalid app optins count: {}", optins.len()));
    }
    let central_optin = &optins[0];
    let slot_optins = &optins[1..(1 + WITHDRAWAL_SLOT_COUNT as usize)];
    submit_apps_optins(&algod, central_optin, slot_optins).await
}

async fn submit_apps_optins(
    algod: &Algod,
    central_optin: &SignedTxFromJs,
    slot_optins: &[SignedTxFromJs],
) -> Result<()> {
    let mut app_optins_txs = vec![signed_js_tx_to_signed_tx1(central_optin)?];
    for slot_optin in slot_optins {
        let slot_optin_tx = signed_js_tx_to_signed_tx1(slot_optin)?;
        app_optins_txs.push(slot_optin_tx);
    }

    let res = algod.broadcast_signed_transactions(&app_optins_txs).await?;
    let _ = wait_for_pending_transaction(&algod, &res.tx_id);

    Ok(())
}
