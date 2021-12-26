use self::withdrawal_history::WithdrawalViewData;
use crate::service::str_to_algos::microalgos_to_algos;
use algonaut::core::MicroAlgos;

pub mod submit_withdraw;
#[allow(clippy::module_inception)]
pub mod withdraw;
pub mod withdrawal_history;

pub fn withdrawal_view_data(
    amount: MicroAlgos,
    description: String,
    date_str: String,
) -> WithdrawalViewData {
    WithdrawalViewData {
        amount: format!("{} Algo", microalgos_to_algos(amount).to_string()),
        description: description.clone(),
        date: date_str.clone(),
        view_id: format!("{}{}", date_str, description),
        amount_not_formatted: amount.to_string(), // microalgos
    }
}
