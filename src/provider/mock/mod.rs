use algonaut::{
    algod::v2::Algod,
    core::{Address, CompiledTeal, MicroAlgos},
    transaction::{Pay, Transaction, TxnBuilder},
    util::sleep,
};
use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::{
    js::{
        common::to_my_algo_tx1,
        js_types_workarounds::{ContractAccountJs, VersionedContractAccountJs},
    },
    model::dao_for_users_view_data::DaoForUsersViewData,
};

pub mod add_roadmap_item_provider_mock;
pub mod app_updates_provider_mock;
pub mod balance_provider_mock;
pub mod buy_shares_provider_mock;
pub mod claim_provider_mock;
pub mod create_assets_provider_mock;
pub mod create_dao_provider_mock;
pub mod dao_user_view_provider_mock;
pub mod drain_provider_mock;
pub mod funds_activity_provider_mock;
pub mod holders_count_provider_mock;
pub mod income_vs_spending_provider_mock;
pub mod investment_provider_mock;
pub mod load_dao_with_id_provider_mock;
pub mod lock_provider_mock;
pub mod my_daos_provider_mock;
pub mod my_shares_provider_mock;
pub mod optin_to_app_provider_mock;
pub mod pay_dao_provider_mock;
pub mod roadmap_provider_mock;
pub mod shares_count_provider_mock;
pub mod shares_distribution_provider_mock;
pub mod unlock_provider_mock;
pub mod update_app_provider_mock;
pub mod update_data_provider_mock;
pub mod view_dao_provider_mock;
pub mod withdraw_provider_mock;
pub mod withdrawal_history_provider_mock;

/// Arbitrary minimal tx for flows that return a tx to be signed in js
/// `address` should (unless trying to cause a signing error) belong to the user using the UI, so they can sign and continue the flow
pub async fn mock_tx(algod: &Algod, address: &Address) -> Result<Transaction> {
    let params = algod.suggested_transaction_params().await?;
    let tx =
        TxnBuilder::with(&params, Pay::new(*address, *address, MicroAlgos(0)).build()).build()?;
    Ok(tx)
}

pub async fn mock_js_tx(algod: &Algod, address: &Address) -> Result<Value> {
    to_my_algo_tx1(&mock_tx(algod, address).await?)
}

pub async fn mock_msgpack_tx(algod: &Algod, address: &Address) -> Result<Vec<u8>> {
    Ok(rmp_serde::to_vec_named(&mock_tx(algod, address).await?)?)
    // .map_err(Error::msg)
}

pub async fn mock_js_txs(algod: &Algod, address: &Address) -> Result<Vec<Value>> {
    Ok(vec![mock_js_tx(algod, address).await?])
}

pub fn mock_address() -> Result<Address> {
    "WJ22SNKZWIDTHIL4MFVOEXKUCKWBQGBPAUFBZHVA7UV2PB6BS4YQKR3EA4"
        .parse()
        .map_err(|_| anyhow!("Unexpected: couldn't parse mock address"))
}

pub fn mock_contract_account() -> Result<VersionedContractAccountJs> {
    Ok(VersionedContractAccountJs {
        version: "1".to_owned(),
        contract: ContractAccountJs {
            address: mock_address()?.to_string(),
            program: CompiledTeal(vec![]),
        },
    })
}

/// simulate a delay doing network requests
pub async fn req_delay() {
    sleep(2000).await
}

pub fn mock_tx_id() -> String {
    "3CUYREVXKFMJOSWJRC3GY6UEAJ3BA36RGN4PKSL7CYRLCWZSIT3A".to_string()
}

pub fn mock_dao_for_users_view_data() -> Result<DaoForUsersViewData> {
    Ok(DaoForUsersViewData {
        name: "Test name".to_owned(),
        description: "Test description".to_owned(),
        share_supply: "123123123".to_owned(),
        share_asset_name: "My asset name".to_owned(),
        share_price: "100".to_owned(),
        share_price_number_algo: "100".to_owned(),
        shares_asset_id: "1231231231".to_owned(),
        logo_url: "https://foo.bar.com/mylogo_url200x300".to_owned(),
        social_media_url: "https://twitter.com/foobardoesntexist".to_owned(),
        app_id: "111112222".to_owned(),
        customer_escrow_address: mock_address()?.to_string(),
        // note that the paths here have to match to what the UI expects, to open the correct views (the parameters/ids can be arbitrary)
        invest_link: format!("/{}", "111112222"),
        my_investment_link: format!("/{}/investment", "111112222"),
        my_investment_link_rel: format!("investment/{}", "111112222"),
        dao_link: format!("/{}", "111112222"),
        creator_address: mock_address()?.to_string(),
    })
}
