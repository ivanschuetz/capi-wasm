use crate::error::FrError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::team_provider::{
    AddTeamMemberParsJs, AddTeamMemberResJs, EditTeamMemberParsJs, EditTeamMemberResJs,
    GetTeamParsJs, GetTeamResJs, SetTeamParsJs, SetTeamResJs, SubmitSetTeamParJs, TeamMemberJs,
    TeamProvider,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::api::fetcher::Fetcher;
use base::dependencies::fetcher;
use base::dev_settings::{submit_dev_settings, DevSettingsSigned};
use base::team::team;
use mbase::dependencies::algod;
use mbase::util::network_util::wait_for_pending_transaction;

pub struct TeamProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TeamProvider for TeamProviderDef {
    async fn get(&self, pars: GetTeamParsJs) -> Result<GetTeamResJs, FrError> {
        let fetcher = fetcher();
        let bytes = fetcher.get(&pars.url).await?;

        let team: Vec<TeamMemberJs> = serde_json::from_slice(&bytes)?;

        Ok(GetTeamResJs { team })
    }

    async fn add_team_member(
        &self,
        pars: AddTeamMemberParsJs,
    ) -> Result<AddTeamMemberResJs, FrError> {
        add_team_member_shared(pars).await
    }

    async fn edit_team_member(
        &self,
        pars: EditTeamMemberParsJs,
    ) -> Result<EditTeamMemberResJs, FrError> {
        edit_team_member_shared(pars).await
    }

    async fn set(&self, pars: SetTeamParsJs) -> Result<SetTeamResJs, FrError> {
        let algod = algod();

        let owner = pars.owner_address.parse().map_err(Error::msg)?;

        let dao_id = pars.dao_id.parse()?;

        let to_sign = team(&algod, &owner, dao_id, &pars.url).await?;

        Ok(SetTeamResJs {
            to_sign: ToSignJs::new(vec![to_sign.app_call_tx])?,
        })
    }

    async fn submit(&self, pars: SubmitSetTeamParJs) -> Result<(), FrError> {
        let algod = algod();

        if pars.txs.len() != 1 {
            return Err(FrError::Internal(format!(
                "Unexpected add roadmap item txs length: {}",
                pars.txs.len()
            )));
        }
        let tx = &pars.txs[0];

        let tx_id = submit_dev_settings(
            &algod,
            &DevSettingsSigned {
                app_call_tx: signed_js_tx_to_signed_tx1(tx)?,
            },
        )
        .await?;

        log::debug!("Submit dev_settings res: {:?}", tx_id);

        let _ = wait_for_pending_transaction(&algod, &tx_id).await?;

        Ok(())
    }
}

/// shared def / mock
pub async fn add_team_member_shared(
    pars: AddTeamMemberParsJs,
) -> Result<AddTeamMemberResJs, FrError> {
    let mut members = pars.existing_members;
    members.push(pars.inputs.to_team_member().into());

    let team_to_save = serde_json::to_string(&members)?;

    Ok(AddTeamMemberResJs {
        team: members,
        to_save: team_to_save,
    })
}

/// shared def / mock
pub async fn edit_team_member_shared(
    pars: EditTeamMemberParsJs,
) -> Result<EditTeamMemberResJs, FrError> {
    let mut members = pars.existing_members;
    let edited_member = pars.inputs;

    if let Some(index) = members.iter().position(|m| m.uuid == edited_member.uuid) {
        members[index] = edited_member;
    } else {
        return Err(FrError::Internal(format!(
            "Invalid state: edited team member must be in existing members"
        )));
    }

    let team_to_save = serde_json::to_string(&members)?;

    Ok(EditTeamMemberResJs {
        team: members,
        to_save: team_to_save,
    })
}
