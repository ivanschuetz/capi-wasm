use self::withdrawal_history::WithdrawalViewData;
use crate::{dependencies::explorer_base_url, service::str_to_algos::microalgos_to_algos};
use algonaut::core::MicroAlgos;

use super::explorer_links::explorer_tx_id_link;

pub mod submit_withdraw;
#[allow(clippy::module_inception)]
pub mod withdraw;
pub mod withdrawal_history;

pub fn withdrawal_view_data(
    amount: MicroAlgos,
    description: String,
    date_str: String,
    tx_id: String,
) -> WithdrawalViewData {
    WithdrawalViewData {
        amount: format!("{} Algo", microalgos_to_algos(amount).to_string()),
        description,
        date: date_str,
        tx_id: tx_id.clone(),
        tx_link: explorer_tx_id_link(explorer_base_url(), &tx_id),
        amount_not_formatted: amount.to_string(), // microalgos
    }
}
