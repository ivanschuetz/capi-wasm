use crate::{
    dependencies::funds_asset_specs,
    js::common::{signed_js_tx_to_signed_tx1, SignedTxFromJs},
};
use algonaut::{algod::v2::Algod, core::Address};
use anyhow::Result;
use base::{
    capi_deps::CapiAssetDaoDeps,
    flows::{
        create_dao::model::Dao,
        drain::drain::{
            fetch_drain_amount_and_drain, submit_drain, to_drain_amounts, DrainSigned, DrainToSign,
        },
    },
    network_util::wait_for_pending_transaction,
};
use mbase::models::funds::FundsAssetId;

/// Returns txs if needed to drain, None if not needed.
pub async fn drain_if_needed_tx(
    algod: &Algod,
    dao: &Dao,
    sender: &Address,
    funds_asset_id: FundsAssetId,
    capi_deps: &CapiAssetDaoDeps,
) -> Result<Option<DrainToSign>> {
    let to_drain = to_drain_amounts(
        algod,
        capi_deps.escrow_percentage,
        funds_asset_id,
        dao.app_id,
    )
    .await?;

    if to_drain.has_something_to_drain() {
        log::debug!("There's an amount to drain: {:?}", to_drain.dao);

        Ok(Some(
            fetch_drain_amount_and_drain(
                algod,
                sender,
                dao.app_id,
                funds_asset_specs()?.id,
                capi_deps,
            )
            .await?,
        ))
    } else {
        Ok(None)
    }
}

pub async fn prepare_pars_and_submit_drain(
    algod: &Algod,
    app_call_js: &SignedTxFromJs,
) -> Result<()> {
    log::debug!("Submit drain txs..");

    let app_call = signed_js_tx_to_signed_tx1(app_call_js)?;

    let drain_tx_id = submit_drain(
        algod,
        &DrainSigned {
            app_call_tx_signed: app_call,
        },
    )
    .await?;
    log::debug!("Submit drain tx id: {:?}", drain_tx_id);

    wait_for_pending_transaction(algod, &drain_tx_id).await?;

    Ok(())
}
