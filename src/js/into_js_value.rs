// TODO is there a better way to do these conversions (avoid panic)
// tsify does it for us for the parameters, so not sure why we've to do it for results

use crate::{
    model::dao_js::DaoJs,
    provider::{
        add_roadmap_item_provider::{AddRoadmapItemResJs, SubmitAddRoadmapItemResJs},
        app_updates_provider::CheckForUpdatesResJs,
        balance_provider::{BalanceChangeResJs, BalanceResJs},
        buy_shares::{InvestResJs, SubmitBuySharesResJs},
        calculate_total_price::{CalculateMaxFundsResJs, CalculateTotalPriceResJs},
        claim_provider::{ClaimResJs, SubmitClaimResJs},
        create_assets_provider::CreateDaoAssetsResJs,
        create_dao_provider::{CreateDaoRes, CreateDaoResJs},
        def::dev_provider_def::{DevSettingsResJs, SubmitDevSettingsResJs},
        drain_provider::{DrainResJs, SubmitDrainResJs},
        funds_activity_provider::LoadFundsActivityResJs,
        funds_raising_provider::FundsRaisingResJs,
        holders_count_provider::{HoldersChangeResJs, HoldersCountResJs},
        income_vs_spending_provider::IncomeVsSpendingResJs,
        investment_provider::{AvailableSharesResJs, LoadInvestorResJs},
        lock_provider::{LockResJs, SubmitLockResJs},
        my_daos_provider::MyDaosResJs,
        my_shares_provider::MySharesResJs,
        optin_to_app_provider::OptInToAppResJs,
        pay_dao_provider::{PayDaoResJs, SubmitPayDaoResJs},
        reclaim_provider::{ReclaimResJs, SubmitReclaimResJs},
        rekey_provider::{RekeyResJs, SubmitRekeyResJs},
        roadmap_provider::GetRoadmapResJs,
        shares_distribution_provider::SharedDistributionResJs,
        team_provider::{AddTeamMemberResJs, EditTeamMemberResJs, GetTeamResJs, SetTeamResJs},
        unlock_provider::{SubmitUnlockResJs, UnlockResJs},
        update_app_provider::{SubmitUpdateAppResJs, UpdateDaoAppResJs},
        update_data_provider::{UpdatableDataResJs, UpdateDataResJs},
        view_dao_provider::ViewDaoResJs,
        withdraw_provider::{SubmitWithdrawResJs, WithdrawResJs},
        withdrawal_history_provider::LoadWithdrawalResJs,
        wyre_provider::WyreReserveResJs,
    },
    service::wallet_connect_tx::WalletConnectTx,
};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use std::fmt::Debug;
use wasm_bindgen::JsValue;

use super::to_sign_js::ToSignJs;

impl From<FundsRaisingResJs> for JsValue {
    fn from(res: FundsRaisingResJs) -> Self {
        to_js(res)
    }
}

impl From<CreateDaoAssetsResJs> for JsValue {
    fn from(res: CreateDaoAssetsResJs) -> Self {
        to_js(res)
    }
}

impl From<CreateDaoResJs> for JsValue {
    fn from(res: CreateDaoResJs) -> Self {
        to_js(res)
    }
}

impl From<CreateDaoRes> for JsValue {
    fn from(res: CreateDaoRes) -> Self {
        to_js(res)
    }
}

impl From<LoadFundsActivityResJs> for JsValue {
    fn from(res: LoadFundsActivityResJs) -> Self {
        to_js(res)
    }
}
impl From<BalanceResJs> for JsValue {
    fn from(res: BalanceResJs) -> Self {
        to_js(res)
    }
}

impl From<InvestResJs> for JsValue {
    fn from(res: InvestResJs) -> Self {
        to_js(res)
    }
}

impl From<DaoJs> for JsValue {
    fn from(res: DaoJs) -> Self {
        to_js(res)
    }
}

impl From<OptInToAppResJs> for JsValue {
    fn from(res: OptInToAppResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitBuySharesResJs> for JsValue {
    fn from(res: SubmitBuySharesResJs) -> Self {
        to_js(res)
    }
}

impl From<ClaimResJs> for JsValue {
    fn from(res: ClaimResJs) -> Self {
        to_js(res)
    }
}

impl From<WalletConnectTx> for JsValue {
    fn from(res: WalletConnectTx) -> Self {
        to_js(res)
    }
}

impl From<ToSignJs> for JsValue {
    fn from(res: ToSignJs) -> Self {
        to_js(res)
    }
}

impl From<AvailableSharesResJs> for JsValue {
    fn from(res: AvailableSharesResJs) -> Self {
        to_js(res)
    }
}

impl From<LoadInvestorResJs> for JsValue {
    fn from(res: LoadInvestorResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitClaimResJs> for JsValue {
    fn from(res: SubmitClaimResJs) -> Self {
        to_js(res)
    }
}

impl From<LockResJs> for JsValue {
    fn from(res: LockResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitLockResJs> for JsValue {
    fn from(res: SubmitLockResJs) -> Self {
        to_js(res)
    }
}

impl From<PayDaoResJs> for JsValue {
    fn from(res: PayDaoResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitPayDaoResJs> for JsValue {
    fn from(res: SubmitPayDaoResJs) -> Self {
        to_js(res)
    }
}

impl From<HoldersCountResJs> for JsValue {
    fn from(res: HoldersCountResJs) -> Self {
        to_js(res)
    }
}

impl From<HoldersChangeResJs> for JsValue {
    fn from(res: HoldersChangeResJs) -> Self {
        to_js(res)
    }
}

impl From<IncomeVsSpendingResJs> for JsValue {
    fn from(res: IncomeVsSpendingResJs) -> Self {
        to_js(res)
    }
}

impl From<MyDaosResJs> for JsValue {
    fn from(res: MyDaosResJs) -> Self {
        to_js(res)
    }
}

impl From<MySharesResJs> for JsValue {
    fn from(res: MySharesResJs) -> Self {
        to_js(res)
    }
}

impl From<SharedDistributionResJs> for JsValue {
    fn from(res: SharedDistributionResJs) -> Self {
        to_js(res)
    }
}

impl From<GetRoadmapResJs> for JsValue {
    fn from(res: GetRoadmapResJs) -> Self {
        to_js(res)
    }
}

impl From<AddRoadmapItemResJs> for JsValue {
    fn from(res: AddRoadmapItemResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitAddRoadmapItemResJs> for JsValue {
    fn from(res: SubmitAddRoadmapItemResJs) -> Self {
        to_js(res)
    }
}

impl From<UnlockResJs> for JsValue {
    fn from(res: UnlockResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitUnlockResJs> for JsValue {
    fn from(res: SubmitUnlockResJs) -> Self {
        to_js(res)
    }
}

impl From<CheckForUpdatesResJs> for JsValue {
    fn from(res: CheckForUpdatesResJs) -> Self {
        to_js(res)
    }
}

impl From<UpdateDaoAppResJs> for JsValue {
    fn from(res: UpdateDaoAppResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitUpdateAppResJs> for JsValue {
    fn from(res: SubmitUpdateAppResJs) -> Self {
        to_js(res)
    }
}

impl From<UpdatableDataResJs> for JsValue {
    fn from(res: UpdatableDataResJs) -> Self {
        to_js(res)
    }
}

impl From<UpdateDataResJs> for JsValue {
    fn from(res: UpdateDataResJs) -> Self {
        to_js(res)
    }
}

impl From<DrainResJs> for JsValue {
    fn from(res: DrainResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitDrainResJs> for JsValue {
    fn from(res: SubmitDrainResJs) -> Self {
        to_js(res)
    }
}

impl From<ViewDaoResJs> for JsValue {
    fn from(res: ViewDaoResJs) -> Self {
        to_js(res)
    }
}

impl From<WithdrawResJs> for JsValue {
    fn from(res: WithdrawResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitWithdrawResJs> for JsValue {
    fn from(res: SubmitWithdrawResJs) -> Self {
        to_js(res)
    }
}

impl From<LoadWithdrawalResJs> for JsValue {
    fn from(res: LoadWithdrawalResJs) -> Self {
        to_js(res)
    }
}

impl From<CalculateTotalPriceResJs> for JsValue {
    fn from(res: CalculateTotalPriceResJs) -> Self {
        to_js(res)
    }
}

impl From<CalculateMaxFundsResJs> for JsValue {
    fn from(res: CalculateMaxFundsResJs) -> Self {
        to_js(res)
    }
}

impl From<BalanceChangeResJs> for JsValue {
    fn from(res: BalanceChangeResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitReclaimResJs> for JsValue {
    fn from(res: SubmitReclaimResJs) -> Self {
        to_js(res)
    }
}

impl From<ReclaimResJs> for JsValue {
    fn from(res: ReclaimResJs) -> Self {
        to_js(res)
    }
}

impl From<WyreReserveResJs> for JsValue {
    fn from(res: WyreReserveResJs) -> Self {
        to_js(res)
    }
}

impl From<RekeyResJs> for JsValue {
    fn from(res: RekeyResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitRekeyResJs> for JsValue {
    fn from(res: SubmitRekeyResJs) -> Self {
        to_js(res)
    }
}

impl From<DevSettingsResJs> for JsValue {
    fn from(res: DevSettingsResJs) -> Self {
        to_js(res)
    }
}

impl From<SubmitDevSettingsResJs> for JsValue {
    fn from(res: SubmitDevSettingsResJs) -> Self {
        to_js(res)
    }
}

impl From<GetTeamResJs> for JsValue {
    fn from(res: GetTeamResJs) -> Self {
        to_js(res)
    }
}

impl From<AddTeamMemberResJs> for JsValue {
    fn from(res: AddTeamMemberResJs) -> Self {
        to_js(res)
    }
}

impl From<EditTeamMemberResJs> for JsValue {
    fn from(res: EditTeamMemberResJs) -> Self {
        to_js(res)
    }
}

impl From<SetTeamResJs> for JsValue {
    fn from(res: SetTeamResJs) -> Self {
        to_js(res)
    }
}

fn to_js<T: Serialize + Debug>(obj: T) -> JsValue {
    let res = to_value(&obj);
    match res {
        Ok(val) => val,
        Err(e) => panic!(
            "Unexpected: couldn't serialize obj to JsValue. Err: {:?}, obj: {:?}",
            e, obj
        ),
    }
}
