use crate::js::{common::SignedTxFromJs, to_sign_js::ToSignJs};
use anyhow::Result;
use async_trait::async_trait;
use base::team::TeamMember;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TeamProvider {
    async fn get(&self, pars: GetTeamParsJs) -> Result<GetTeamResJs>;

    async fn add_team_member(&self, pars: AddTeamMemberParsJs) -> Result<AddTeamMemberResJs>;
    async fn edit_team_member(&self, pars: EditTeamMemberParsJs) -> Result<EditTeamMemberResJs>;

    async fn set(&self, pars: SetTeamParsJs) -> Result<SetTeamResJs>;
    async fn submit(&self, pars: SubmitSetTeamParJs) -> Result<()>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTeamParsJs {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeamMemberInputs {
    pub name: String,
    pub descr: String,
    pub role: String,
    pub picture: String,
    pub social_links: Vec<String>,
}

impl TeamMemberInputs {
    pub fn to_team_member(&self) -> TeamMember {
        TeamMember {
            uuid: Uuid::new_v4().to_string(),
            name: self.name.clone(),
            descr: self.descr.clone(),
            role: self.role.clone(),
            picture: self.picture.clone(),
            social_links: self.social_links.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTeamResJs {
    pub team: Vec<TeamMember>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTeamMemberParsJs {
    pub inputs: TeamMemberInputs, // directly here ok since it's just strings currently
    pub existing_members: Vec<TeamMember>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddTeamMemberResJs {
    pub team: Vec<TeamMember>, // display
    pub to_save: String,       // upload to IPFS
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditTeamMemberParsJs {
    pub inputs: TeamMember,
    pub existing_members: Vec<TeamMember>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditTeamMemberResJs {
    pub team: Vec<TeamMember>, // display
    pub to_save: String,       // upload to IPFS
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetTeamParsJs {
    pub dao_id: String,
    pub owner_address: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetTeamResJs {
    pub to_sign: ToSignJs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitSetTeamParJs {
    pub txs: Vec<SignedTxFromJs>,
}
