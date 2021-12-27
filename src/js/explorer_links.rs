pub fn explorer_tx_id_link(base_url: &str, tx_id: &str) -> String {
    format!("{}/tx/{}", base_url, tx_id)
}
