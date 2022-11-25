use algonaut::algod::v2::Algod;
use anyhow::{anyhow, Result};
use base::{
    flows::{create_dao::model::Dao, drain::drain::calculate_dao_and_capi_escrow_xfer_amounts},
    state::account_state::funds_holdings,
};
use mbase::{
    models::{
        capi_deps::CapiAssetDaoDeps,
        funds::{FundsAmount, FundsAssetId},
    },
    state::dao_app_state::dao_global_state,
};

// The "actual" funds of the app: available funds + not-yet-drained funds(deducting the fee).
// Available/not available funds distinction is internal / low level.
// This is the dao's balance users see on the UI.
pub async fn owned_funds(
    algod: &Algod,
    dao: &Dao,
    funds_asset_id: FundsAssetId,
    capi_deps: &CapiAssetDaoDeps,
) -> Result<FundsAmount> {
    let holdings = funds_holdings(algod, &dao.app_address(), funds_asset_id).await?;
    let available_funds = dao_global_state(algod, dao.app_id).await?.available;

    let not_available_funds = FundsAmount::new(
        holdings
            .val()
            .checked_sub(available_funds.val())
            .ok_or_else(|| anyhow!("Error sub: {:?} - {:?}", holdings, available_funds))?,
    );
    // the drained amount (made available to the dao) if we drained now
    let calculated_drained_amount = calculate_dao_and_capi_escrow_xfer_amounts(
        not_available_funds,
        capi_deps.escrow_percentage,
    )?
    .dao;

    Ok(FundsAmount::new(
        available_funds
            .val()
            .checked_add(calculated_drained_amount.val())
            .ok_or_else(|| {
                anyhow!(
                    "Error add: {:?} + {:?}",
                    available_funds,
                    calculated_drained_amount
                )
            })?,
    ))
}
