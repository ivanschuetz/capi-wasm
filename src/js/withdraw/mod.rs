use self::withdrawal_history::WithdrawalViewData;
use super::explorer_links::explorer_tx_id_link_env;
use crate::{
    dependencies::FundsAssetSpecs, service::str_to_algos::base_units_to_display_units_str,
};
use core::{flows::create_dao::storage::load_dao::TxId, funds::FundsAmount};

pub mod submit_withdraw;
#[allow(clippy::module_inception)]
pub mod withdraw;
pub mod withdrawal_history;

pub fn withdrawal_view_data(
    amount: FundsAmount,
    funds_asset_specs: &FundsAssetSpecs,
    description: String,
    date_str: String,
    tx_id: TxId,
) -> WithdrawalViewData {
    WithdrawalViewData {
        amount: base_units_to_display_units_str(amount, funds_asset_specs),
        description,
        date: date_str,
        tx_id: tx_id.to_string(),
        tx_link: explorer_tx_id_link_env(&tx_id),
        amount_not_formatted: amount.to_string(), // microalgos
    }
}
