use super::{
    add_roadmap_item_provider::AddRoadmapItemProvider,
    app_updates_provider::AppUpdatesProvider,
    balance_provider::BalanceProvider,
    buy_shares::BuySharesProvider,
    calculate_total_price::CalculateTotalPriceProvider,
    claim_provider::ClaimProvider,
    create_assets_provider::CreateAssetsProvider,
    create_dao_provider::CreateDaoProvider,
    dao_provider::DaoProvider,
    def::{
        add_roadmap_item_provider_def::AddRoadmapItemProviderDef,
        app_updates_provider_def::AppUpdatesProviderDef, balance_provider_def::BalanceProviderDef,
        buy_shares_provider_def::BuySharesProviderDef,
        calculate_total_price_def::CalculateTotalPriceDef, claim_provider_def::ClaimProviderDef,
        create_assets_provider_def::CreateAssetsProviderDef,
        create_dao_provider_def::CreateDaoProviderDef, dao_provider_def::DaoUserViewProviderDef,
        description_provider_def::DescriptionProviderDef, dev_provider_def::DevProviderDef,
        dividends_provider_def::DividendsProviderDef, drain_provider_def::DrainProviderDef,
        funds_activity_provider_def::FundsActivityProviderDef,
        funds_raising_provider_def::FundsRaisingProviderDef, hash_provider_def::HashProviderDef,
        holders_count_provider_def::HoldersCountProviderDef,
        income_vs_spending_provider_def::IncomeVsSpendingProviderDef,
        investment_provider_def::InvestmentProviderDef, lock_provider_def::LockProviderDef,
        metadata_provider_def::MetadataProviderDef, my_daos_provider_def::MyDaosProviderDef,
        my_shares_provider_def::MySharesProviderDef,
        optin_to_app_provider_def::OptinToAppProviderDef, pay_dao_provider_def::PayDaoProviderDef,
        reclaim_provider_def::ReclaimProviderDef, rekey_provider_def::RekeyProviderDef,
        roadmap_provider_def::RoadmapProviderDef,
        shares_count_provider_def::SharesCountProviderDef,
        shares_distribution_provider_def::SharesDistributionProviderDef,
        team_provider_def::TeamProviderDef, unlock_provider_def::UnlockProviderDef,
        update_app_provider_def::UpdateAppProviderDef,
        update_data_provider_def::UpdateDataProviderDef, view_dao_provider_def::ViewDaoProviderDef,
        withdraw_provider_def::WithdrawProviderDef,
        withdrawal_history_provider_def::WithdrawalHistoryProviderDef,
        wyre_provider_def::WyreProviderDef,
    },
    description_provider::DescriptionProvider,
    dividends_provider::DividendsProvider,
    drain_provider::DrainProvider,
    funds_activity_provider::FundsActivityProvider,
    funds_raising_provider::FundsRaisingProvider,
    holders_count_provider::HoldersCountProvider,
    income_vs_spending_provider::IncomeVsSpendingProvider,
    investment_provider::InvestmentProvider,
    lock_provider::LockProvider,
    mock::{
        add_roadmap_item_provider_mock::AddRoadmapItemProviderMock,
        app_updates_provider_mock::AppUpdatesProviderMock,
        balance_provider_mock::BalanceProviderMock,
        buy_shares_provider_mock::BuySharesProviderMock,
        calculate_total_price_mock::CalculateTotalPriceMock,
        claim_provider_mock::ClaimProviderMock,
        create_assets_provider_mock::CreateAssetsProviderMock,
        create_dao_provider_mock::CreateDaoProviderMock,
        dao_provider_mock::DaoUserViewProviderMock,
        description_provider_mock::DescriptionProviderMock,
        dividends_provider_mock::DividendsProviderMock, drain_provider_mock::DrainProviderMock,
        funds_activity_provider_mock::FundsActivityProviderMock,
        funds_raising_provider_mock::FundsRaisingProviderMock,
        holders_count_provider_mock::HoldersCountProviderMock,
        income_vs_spending_provider_mock::IncomeVsSpendingProviderMock,
        investment_provider_mock::InvestmentProviderMock, lock_provider_mock::LockProviderMock,
        my_daos_provider_mock::MyDaosProviderMock, my_shares_provider_mock::MySharesProviderMock,
        optin_to_app_provider_mock::OptinToAppProviderMock,
        pay_dao_provider_mock::PayDaoProviderMock, reclaim_provider_mock::ReclaimProviderMock,
        rekey_provider_mock::RekeyProviderMock, roadmap_provider_mock::RoadmapProviderMock,
        shares_count_provider_mock::SharesCountProviderMock,
        shares_distribution_provider_mock::SharesDistributionProviderMock,
        team_provider_mock::TeamProviderMock, unlock_provider_mock::UnlockProviderMock,
        update_app_provider_mock::UpdateAppProviderMock,
        update_data_provider_mock::UpdateDataProviderMock,
        view_dao_provider_mock::ViewDaoProviderMock, withdraw_provider_mock::WithdrawProviderMock,
        withdrawal_history_provider_mock::WithdrawalHistoryProviderMock,
        wyre_provider_mock::WyreProviderMock,
    },
    my_daos_provider::MyDaosProvider,
    my_shares_provider::MySharesProvider,
    optin_to_app_provider::OptinToAppProvider,
    pay_dao_provider::PayDaoProvider,
    reclaim_provider::ReclaimProvider,
    rekey_provider::RekeyProvider,
    roadmap_provider::RoadmapProvider,
    shares_count_provider::SharesCountProvider,
    shares_distribution_provider::SharesDistributionProvider,
    team_provider::TeamProvider,
    unlock_provider::UnlockProvider,
    update_app_provider::UpdateAppProvider,
    update_data_provider::UpdateDataProvider,
    view_dao_provider::ViewDaoProvider,
    withdraw_provider::WithdrawProvider,
    withdrawal_history_provider::WithdrawalHistoryProvider,
    wyre_provider::WyreProvider,
};
use crate::{dependencies::data_type, js::common::to_js_value};
use mbase::dependencies::DataType;
use wasm_bindgen::JsValue;

pub struct Providers<'a> {
    pub funds_activity: &'a dyn FundsActivityProvider,
    pub balance: &'a dyn BalanceProvider,
    pub buy_shares: &'a dyn BuySharesProvider,
    pub shares_count: &'a dyn SharesCountProvider,
    pub dao: &'a dyn DaoProvider,
    pub app_optin: &'a dyn OptinToAppProvider,
    pub claim: &'a dyn ClaimProvider,
    pub investment: &'a dyn InvestmentProvider,
    pub lock: &'a dyn LockProvider,
    pub pay_dao: &'a dyn PayDaoProvider,
    pub holders_count: &'a dyn HoldersCountProvider,
    pub income_vs_spending: &'a dyn IncomeVsSpendingProvider,
    pub my_daos: &'a dyn MyDaosProvider,
    pub my_shares: &'a dyn MySharesProvider,
    pub shares_distribution: &'a dyn SharesDistributionProvider,
    pub add_roadmap_item: &'a dyn AddRoadmapItemProvider,
    pub roadmap: &'a dyn RoadmapProvider,
    pub unlock: &'a dyn UnlockProvider,
    pub app_updates: &'a dyn AppUpdatesProvider,
    pub update_app: &'a dyn UpdateAppProvider,
    pub update_data: &'a dyn UpdateDataProvider,
    pub view_dao: &'a dyn ViewDaoProvider,
    pub drain: &'a dyn DrainProvider,
    pub withdraw: &'a dyn WithdrawProvider,
    pub withdrawals_history: &'a dyn WithdrawalHistoryProvider, // remove ? seems not to be used anymore (route/comp in react, but not used)
    pub create_dao: &'a dyn CreateDaoProvider,
    pub create_assets: &'a dyn CreateAssetsProvider,
    pub calculate_total_price: &'a dyn CalculateTotalPriceProvider,
    pub dividend: &'a dyn DividendsProvider,
    pub reclaim: &'a dyn ReclaimProvider,
    pub description: &'a dyn DescriptionProvider,
    pub wyre: &'a dyn WyreProvider,
    pub rekey: &'a dyn RekeyProvider,
    pub raised: &'a dyn FundsRaisingProvider,
    pub hash: HashProviderDef,
    pub metadata: MetadataProviderDef,
    pub dev_settings: DevProviderDef,
    pub team: &'a dyn TeamProvider,
}

// we return JsValue for convenience, this is used only in the bridge (which returns JsValue)
pub fn providers<'a>() -> Result<Providers<'a>, JsValue> {
    // note that we create data_type here instead of parametrizing, it's noise as all the bridge functions would have to pass it and no good reason for it.
    let data_type = data_type().map_err(to_js_value)?;
    log::info!("Data type config: {data_type:?}");
    Ok(match data_type {
        DataType::Real => def_providers(),
        DataType::Mock => mock_providers(),
    })
}

fn def_providers<'a>() -> Providers<'a> {
    Providers {
        funds_activity: &FundsActivityProviderDef {},
        balance: &BalanceProviderDef {},
        buy_shares: &BuySharesProviderDef {},
        shares_count: &SharesCountProviderDef {},
        dao: &DaoUserViewProviderDef {},
        app_optin: &OptinToAppProviderDef {},
        claim: &ClaimProviderDef {},
        investment: &InvestmentProviderDef {},
        lock: &LockProviderDef {},
        pay_dao: &PayDaoProviderDef {},
        holders_count: &HoldersCountProviderDef {},
        income_vs_spending: &IncomeVsSpendingProviderDef {},
        my_daos: &MyDaosProviderDef {},
        my_shares: &MySharesProviderDef {},
        shares_distribution: &SharesDistributionProviderDef {},
        add_roadmap_item: &AddRoadmapItemProviderDef {},
        roadmap: &RoadmapProviderDef {},
        unlock: &UnlockProviderDef {},
        app_updates: &AppUpdatesProviderDef {},
        update_app: &UpdateAppProviderDef {},
        update_data: &UpdateDataProviderDef {},
        view_dao: &ViewDaoProviderDef {},
        drain: &DrainProviderDef {},
        withdraw: &WithdrawProviderDef {},
        withdrawals_history: &WithdrawalHistoryProviderDef {},
        create_dao: &CreateDaoProviderDef {},
        create_assets: &CreateAssetsProviderDef {},
        calculate_total_price: &CalculateTotalPriceDef {},
        dividend: &DividendsProviderDef {},
        reclaim: &ReclaimProviderDef {},
        description: &DescriptionProviderDef {},
        wyre: &WyreProviderDef {},
        rekey: &RekeyProviderDef {},
        raised: &FundsRaisingProviderDef {},
        hash: HashProviderDef {},
        metadata: MetadataProviderDef {},
        dev_settings: DevProviderDef {},
        team: &TeamProviderDef {},
    }
}

fn mock_providers<'a>() -> Providers<'a> {
    Providers {
        funds_activity: &FundsActivityProviderMock {},
        balance: &BalanceProviderMock {},
        buy_shares: &BuySharesProviderMock {},
        shares_count: &SharesCountProviderMock {},
        dao: &DaoUserViewProviderMock {},
        app_optin: &OptinToAppProviderMock {},
        claim: &ClaimProviderMock {},
        investment: &InvestmentProviderMock {},
        lock: &LockProviderMock {},
        pay_dao: &PayDaoProviderMock {},
        holders_count: &HoldersCountProviderMock {},
        income_vs_spending: &IncomeVsSpendingProviderMock {},
        my_daos: &MyDaosProviderMock {},
        my_shares: &MySharesProviderMock {},
        shares_distribution: &SharesDistributionProviderMock {},
        add_roadmap_item: &AddRoadmapItemProviderMock {},
        roadmap: &RoadmapProviderMock {},
        unlock: &UnlockProviderMock {},
        app_updates: &AppUpdatesProviderMock {},
        update_app: &UpdateAppProviderMock {},
        update_data: &UpdateDataProviderMock {},
        view_dao: &ViewDaoProviderMock {},
        drain: &DrainProviderMock {},
        withdraw: &WithdrawProviderMock {},
        withdrawals_history: &WithdrawalHistoryProviderMock {},
        create_dao: &CreateDaoProviderMock {},
        create_assets: &CreateAssetsProviderMock {},
        calculate_total_price: &CalculateTotalPriceMock {},
        dividend: &DividendsProviderMock {},
        reclaim: &ReclaimProviderMock {},
        description: &DescriptionProviderMock {},
        wyre: &WyreProviderMock {},
        rekey: &RekeyProviderMock {},
        raised: &FundsRaisingProviderMock {},
        hash: HashProviderDef {},
        metadata: MetadataProviderDef {},
        dev_settings: DevProviderDef {},
        team: &TeamProviderMock {},
    }
}
