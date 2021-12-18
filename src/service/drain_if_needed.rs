use algonaut::{algod::v2::Algod, core::Address};
use anyhow::Result;
use core::{
    flows::{
        create_project::model::Project,
        drain::drain::{
            drain_customer_escrow, submit_drain_customer_escrow, DrainCustomerEscrowSigned,
            DrainCustomerEscrowToSign, FIXED_FEE, MIN_BALANCE,
        },
    },
    network_util::wait_for_pending_transaction,
};

use crate::js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs};

/// Returns txs if needed to drain, None if not needed.
pub async fn drain_if_needed_txs(
    algod: &Algod,
    project: &Project,
    sender: &Address,
) -> Result<Option<DrainCustomerEscrowToSign>> {
    let customer_escrow_balance = algod
        .account_information(&project.customer_escrow.address)
        .await?
        .amount;

    // TODO dynamic min balance? (and fee)
    if customer_escrow_balance > (MIN_BALANCE + FIXED_FEE) {
        log::debug!("There's an amount to drain: {}", customer_escrow_balance);
        Ok(Some(
            drain_customer_escrow(
                algod,
                sender,
                project.central_app_id,
                &project.customer_escrow,
                &project.central_escrow,
            )
            .await?,
        ))
    } else {
        Ok(None)
    }
}

pub async fn submit_drain(
    algod: &Algod,
    drain_passthrough_tx: &[u8],
    pay_drain_fee_tx: &SignedTxFromJs,
    drain_app_call_tx: &SignedTxFromJs,
) -> Result<()> {
    log::debug!("Submit drain txs..");

    let drain_tx = rmp_serde::from_slice(drain_passthrough_tx)?;

    let drain_pay_drain_tx_fee_tx = signed_js_tx_to_signed_tx1(pay_drain_fee_tx)?;
    let drain_app_call_tx = signed_js_tx_to_signed_tx1(drain_app_call_tx)?;

    let drain_tx_id = submit_drain_customer_escrow(
        algod,
        &DrainCustomerEscrowSigned {
            drain_tx,
            pay_fee_tx: drain_pay_drain_tx_fee_tx,
            app_call_tx_signed: drain_app_call_tx,
        },
    )
    .await?;
    log::debug!("Submit drain tx id: {:?}", drain_tx_id);

    wait_for_pending_transaction(algod, &drain_tx_id).await?;

    Ok(())
}
