use algonaut::algod::v2::Algod;
use anyhow::Result;
use core::{
    flows::create_dao::model::Dao,
    funds::{FundsAmount, FundsAssetId},
    state::account_state::funds_holdings,
};

pub async fn available_funds(
    algod: &Algod,
    dao: &Dao,
    funds_asset_id: FundsAssetId,
) -> Result<FundsAmount> {
    let customer_escrow_amount =
        funds_holdings(algod, dao.customer_escrow.address(), funds_asset_id).await?;

    let central_escrow_amount =
        funds_holdings(algod, dao.central_escrow.address(), funds_asset_id).await?;

    Ok(customer_escrow_amount + central_escrow_amount)
}
