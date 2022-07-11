use crate::dependencies::funds_asset_specs;
use crate::error::FrError;
use crate::provider::funds_raising_provider::{
    FundsRaisingParsJs, FundsRaisingProvider, FundsRaisingResJs, FundsRaisingState,
    FundsRaisingStateJs,
};
use crate::service::number_formats::base_units_to_display_units_readable;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use mbase::dependencies::algod;
use mbase::models::dao_id::DaoId;
use mbase::state::dao_app_state::{dao_global_state, CentralAppGlobalState};
use mbase::util::decimal_util::DecimalExt;
use rust_decimal::Decimal;

pub struct FundsRaisingProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl FundsRaisingProvider for FundsRaisingProviderDef {
    async fn data(&self, pars: FundsRaisingParsJs) -> Result<FundsRaisingResJs, FrError> {
        let algod = algod();
        let funds_asset_specs = funds_asset_specs()?;

        let dao_id: DaoId = pars.dao_id.parse()?;

        let dao_state = dao_global_state(&algod, dao_id.0).await?;

        let state: FundsRaisingState =
            if Utc::now() >= dao_state.min_funds_target_end_date.to_date()? {
                let percentage = raised_diff_percentage(&dao_state)?;
                if percentage < 1.into() {
                    FundsRaisingState::GoalNotReached
                } else if percentage >= 1.into() && percentage < goal_reached_top()? {
                    FundsRaisingState::GoalReached
                } else {
                    FundsRaisingState::GoalExceeded(percentage_delta(percentage)?)
                }
            } else {
                FundsRaisingState::Raising
            };

        let state_js = match state {
            FundsRaisingState::Raising => FundsRaisingStateJs::Raising,
            FundsRaisingState::GoalReached => FundsRaisingStateJs::GoalReached,
            FundsRaisingState::GoalNotReached => FundsRaisingStateJs::GoalNotReached,
            FundsRaisingState::GoalExceeded(_) => FundsRaisingStateJs::GoalExceeded,
        };

        let exceeded_percentage = match state {
            FundsRaisingState::GoalExceeded(percentage) => Some(percentage),
            _ => None,
        };

        Ok(FundsRaisingResJs {
            raised_number: dao_state.raised.to_string(),
            raised: base_units_to_display_units_readable(dao_state.raised, &funds_asset_specs)?,
            state: state_js,
            goal_exceeded_percentage: exceeded_percentage.map(|e| e.format_percentage()), // Some("40%".to_owned()),
        })
    }
}

fn goal_reached_top() -> Result<Decimal, FrError> {
    Ok(Decimal::from_f64_retain(1.1).ok_or_else(|| {
        FrError::Msg("Unexpected error converting hardcoded value to decimal".to_owned())
    })?)
}

fn percentage_delta(percentage: Decimal) -> Result<Decimal, FrError> {
    Ok(percentage
        .checked_sub(1.into())
        .ok_or_else(|| FrError::Msg(format!("Unexpected error sub: {percentage:?} - 1")))?)
}

fn raised_diff_percentage(dao_state: &CentralAppGlobalState) -> Result<Decimal, FrError> {
    Ok((dao_state.raised.as_decimal())
        .checked_div(dao_state.min_funds_target.as_decimal())
        .ok_or_else(|| {
            FrError::Msg(format!(
                "Error div: {:?} / {:?}",
                dao_state.raised, dao_state.min_funds_target
            ))
        })?)
}
