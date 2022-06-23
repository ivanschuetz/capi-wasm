use algonaut::algod::v2::Algod;
use anyhow::Result;
use base::{flows::create_dao::model::Dao, state::account_state::funds_holdings};
use mbase::models::funds::{FundsAmount, FundsAssetId};

pub async fn available_funds(
    algod: &Algod,
    dao: &Dao,
    funds_asset_id: FundsAssetId,
) -> Result<FundsAmount> {
    funds_holdings(algod, &dao.app_address(), funds_asset_id).await
}
