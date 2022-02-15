use algonaut::algod::v2::Algod;
use anyhow::Result;
use core::{
    flows::create_project::model::Project,
    funds::{FundsAmount, FundsAssetId},
    state::account_state::funds_holdings,
};

pub async fn available_funds(
    algod: &Algod,
    project: &Project,
    funds_asset_id: FundsAssetId,
) -> Result<FundsAmount> {
    let customer_escrow_amount =
        funds_holdings(algod, project.customer_escrow.address(), funds_asset_id).await?;

    let central_escrow_amount =
        funds_holdings(algod, project.central_escrow.address(), funds_asset_id).await?;

    Ok(customer_escrow_amount + central_escrow_amount)
}
