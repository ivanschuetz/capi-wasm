use crate::{
    dependencies::funds_asset_specs,
    js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs},
};
use algonaut::{algod::v2::Algod, core::Address};
use anyhow::Result;
use base::{
    capi_asset::capi_asset_dao_specs::CapiAssetDaoDeps,
    flows::{
        create_dao::model::Dao,
        drain::drain::{
            fetch_drain_amount_and_drain, submit_drain_customer_escrow, DrainCustomerEscrowSigned,
            DrainCustomerEscrowToSign,
        },
    },
    funds::FundsAssetId,
    network_util::wait_for_pending_transaction,
    state::account_state::funds_holdings,
};

/// Returns txs if needed to drain, None if not needed.
pub async fn drain_if_needed_txs(
    algod: &Algod,
    dao: &Dao,
    sender: &Address,
    funds_asset_id: FundsAssetId,
    capi_deps: &CapiAssetDaoDeps,
) -> Result<Option<DrainCustomerEscrowToSign>> {
    let customer_escrow_amount =
        funds_holdings(algod, dao.customer_escrow.address(), funds_asset_id).await?;

    if customer_escrow_amount.0 > 0 {
        log::debug!("There's an amount to drain: {}", customer_escrow_amount);

        Ok(Some(
            fetch_drain_amount_and_drain(
                algod,
                sender,
                dao.app_id,
                funds_asset_specs()?.id,
                capi_deps,
                &dao.customer_escrow.account,
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
    drain_app_call_tx: &SignedTxFromJs,
    capi_share_tx: &[u8],
    capi_app_call_tx_signed: &SignedTxFromJs,
) -> Result<()> {
    log::debug!("Submit drain txs..");

    let drain_tx = rmp_serde::from_slice(drain_passthrough_tx)?;
    let drain_app_call_tx = signed_js_tx_to_signed_tx1(drain_app_call_tx)?;
    let capi_share_tx = rmp_serde::from_slice(capi_share_tx)?;
    let capi_app_call_tx_signed = signed_js_tx_to_signed_tx1(capi_app_call_tx_signed)?;

    let drain_tx_id = submit_drain_customer_escrow(
        algod,
        &DrainCustomerEscrowSigned {
            drain_tx,
            app_call_tx_signed: drain_app_call_tx,
            capi_share_tx,
            capi_app_call_tx_signed,
        },
    )
    .await?;
    log::debug!("Submit drain tx id: {:?}", drain_tx_id);

    wait_for_pending_transaction(algod, &drain_tx_id).await?;

    Ok(())
}
