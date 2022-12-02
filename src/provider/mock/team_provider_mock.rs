use crate::{
    error::FrError,
    provider::{
        def::team_provider_def::{add_team_member_shared, edit_team_member_shared},
        team_provider::{
            AddTeamMemberParsJs, AddTeamMemberResJs, EditTeamMemberParsJs, EditTeamMemberResJs,
            GetTeamParsJs, GetTeamResJs, SetTeamParsJs, SetTeamResJs, SubmitSetTeamParJs,
            TeamMemberJs, TeamProvider,
        },
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
    async fn get(&self, _: GetTeamParsJs) -> Result<GetTeamResJs, FrError> {
        Ok(GetTeamResJs {
            team: vec![TeamMemberJs {
                uuid: "1".to_string(),
                name: "Raelyn Gaines".to_string(),
                descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur".to_string(),
                role: "CEO".to_string(),
                picture: "https://placekitten.com/400/400".to_string(),
                social_links: vec![
                    "https://twitter.com/capi_fin".to_string(),
                    "https://github.com/ivanschuetz".to_string(),
                    "https://www.linkedin.com/in/ivan-schütz-61a8165a/".to_string()
                ]
            }, TeamMemberJs {
                uuid: "2".to_string(),
                name: "Talon Bernard".to_string(),
                descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
                role: "Cofounder".to_string(),
                picture: "https://placekitten.com/400/400".to_string(),
                social_links: vec![
                    "https://twitter.com/capi_fin".to_string(),
                ],
            }, TeamMemberJs {
                uuid: "3".to_string(),
                name: "Barbara Jimenez".to_string(),
                descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.".to_string(),
                role: "Software engineer".to_string(),
                picture: "https://placekitten.com/400/400".to_string(),
                social_links: vec![
                    "https://twitter.com/capi_fin".to_string(),
                    "https://www.linkedin.com/in/ivan-schütz-61a8165a/".to_string(),
                ],
            }, TeamMemberJs {
                uuid: "4".to_string(),
                name: "Colin Vu".to_string(),
                descr: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.".to_string(),
                role: "Marketing".to_string(),
                picture: "https://placekitten.com/400/400".to_string(),
                social_links: vec![
                    "https://twitter.com/capi_fin".to_string(),
                    "https://github.com/ivanschuetz".to_string(),
                ],
            }
            ],
        })
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

        let owner_address = pars.owner_address.parse().map_err(Error::msg)?;

        req_delay().await;

        Ok(SetTeamResJs {
            to_sign: mock_to_sign(&algod, &owner_address).await?,
        })
    }

    async fn submit(&self, _: SubmitSetTeamParJs) -> Result<(), FrError> {
        req_delay().await;

        Ok(())
    }
}
