use crate::{
    dependencies::{funds_asset_specs, FundsAssetSpecs},
    error::FrError,
    provider::withdrawal_history_provider::{
        LoadWithdrawalParJs, LoadWithdrawalResJs, WithdrawalHistoryProvider, WithdrawalViewData,
    },
};
use algonaut::{algod::v2::Algod, indexer::v2::Indexer};
use anyhow::Result;
use async_trait::async_trait;
use base::flows::withdraw::withdrawals::withdrawals;
use mbase::{
    dependencies::{algod, indexer},
    models::dao_id::DaoId,
};

use super::withdraw_provider_def::withdrawal_view_data;

pub struct WithdrawalHistoryProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WithdrawalHistoryProvider for WithdrawalHistoryProviderDef {
    async fn get(&self, pars: LoadWithdrawalParJs) -> Result<LoadWithdrawalResJs, FrError> {
        let algod = algod();
        let indexer = indexer();

        let dao_id = pars.dao_id.parse()?;

        let entries = load_withdrawals(&algod, &indexer, &funds_asset_specs()?, dao_id).await?;

        Ok(LoadWithdrawalResJs { entries })
    }
}

pub async fn load_withdrawals(
    algod: &Algod,
    indexer: &Indexer,
    funds_asset_specs: &FundsAssetSpecs,
    dao_id: DaoId,
) -> Result<Vec<WithdrawalViewData>, FrError> {
    let entries = withdrawals(algod, indexer, dao_id, funds_asset_specs.id, &None, &None).await?;
    let mut reqs_view_data = vec![];
    for entry in entries {
        reqs_view_data.push(withdrawal_view_data(
            entry.amount,
            funds_asset_specs,
            entry.description,
            entry.date.to_rfc2822(),
            entry.tx_id,
        ));
    }
    Ok(reqs_view_data)
}
