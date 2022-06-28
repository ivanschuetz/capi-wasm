use algonaut::algod::v2::Algod;
use anyhow::Result;
use base::{flows::create_dao::model::Dao, state::account_state::funds_holdings};
use mbase::models::funds::{FundsAmount, FundsAssetId};

// TODO rename - total_funds, "available" can be confused with global state available funds
// note that from the ui's POV, they're available, as in, draining is an implementation detail
pub async fn available_funds(
    algod: &Algod,
    dao: &Dao,
    funds_asset_id: FundsAssetId,
) -> Result<FundsAmount> {
    funds_holdings(algod, &dao.app_address(), funds_asset_id).await
}
