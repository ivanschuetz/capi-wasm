use algonaut::algod::v2::Algod;
use anyhow::Result;
use base::{flows::{create_dao::model::Dao, drain::drain::calculate_dao_and_capi_escrow_xfer_amounts}, state::account_state::funds_holdings, capi_deps::CapiAssetDaoDeps};
use mbase::{
    checked::{CheckedAdd, CheckedSub},
    models::funds::{FundsAmount, FundsAssetId},
};

pub async fn available_funds(
    algod: &Algod,
    dao: &Dao,
    funds_asset_id: FundsAssetId,
    capi_deps: &CapiAssetDaoDeps
) -> Result<FundsAmount> {
    let customer_escrow_amount =
        funds_holdings(algod, dao.customer_escrow.address(), funds_asset_id).await?;
    // apply fee in advance - user wants to see funds actually available to the project,
    // the fee will be paid inevitably when funds are accessed (drained), so we should substract it already
    let fee_to_be_paid = calculate_dao_and_capi_escrow_xfer_amounts(customer_escrow_amount, capi_deps.escrow_percentage)?.capi;
    let available_customer_escrow_amount = customer_escrow_amount.sub(&fee_to_be_paid)?;

    let app_amount = funds_holdings(algod, &dao.app_address(), funds_asset_id).await?;

    available_customer_escrow_amount.add(&app_amount)
}
