use core::flows::create_project::storage::load_project::TxId;

use algonaut::core::Address;

use crate::dependencies::explorer_base_url;

/// Base URL determined by environment variable
pub fn explorer_tx_id_link_env(tx_id: &TxId) -> String {
    explorer_tx_id_link(explorer_base_url(), tx_id)
}

pub fn explorer_tx_id_link(base_url: &str, tx_id: &TxId) -> String {
    format!("{}tx/{}", base_url, tx_id.to_string())
}

/// Base URL determined by environment variable
pub fn explorer_address_link_env(address: &Address) -> String {
    explorer_address_link(explorer_base_url(), address)
}

pub fn explorer_address_link(base_url: &str, address: &Address) -> String {
    format!("{}address/{}", base_url, address)
}
