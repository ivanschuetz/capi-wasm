use crate::provider::{
    def::team_provider_def::{add_team_member_shared, edit_team_member_shared},
    team_provider::{
        AddTeamMemberParsJs, AddTeamMemberResJs, EditTeamMemberParsJs, EditTeamMemberResJs,
        GetTeamParsJs, GetTeamResJs, SetTeamParsJs, SetTeamResJs, SubmitSetTeamParJs, TeamProvider,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use mbase::dependencies::algod;

use super::{mock_to_sign, req_delay};

pub struct TeamProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TeamProvider for TeamProviderMock {
    async fn get(&self, _: GetTeamParsJs) -> Result<GetTeamResJs> {
        Ok(GetTeamResJs { team: vec![] })
    }

    async fn add_team_member(&self, pars: AddTeamMemberParsJs) -> Result<AddTeamMemberResJs> {
        add_team_member_shared(pars).await
    }

    async fn edit_team_member(&self, pars: EditTeamMemberParsJs) -> Result<EditTeamMemberResJs> {
        edit_team_member_shared(pars).await
    }

    async fn set(&self, pars: SetTeamParsJs) -> Result<SetTeamResJs> {
        let algod = algod();

        let owner_address = pars.owner_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(SetTeamResJs {
            to_sign: mock_to_sign(&algod, &owner_address).await?,
        })
    }

    async fn submit(&self, _: SubmitSetTeamParJs) -> Result<()> {
        req_delay().await;

        Ok(())
    }
}
