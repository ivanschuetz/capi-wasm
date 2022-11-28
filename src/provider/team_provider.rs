use crate::{
    error::FrError,
    js::{bridge::log_wrap_new, common::SignedTxFromJs, to_sign_js::ToSignJs},
};
use anyhow::Result;
use async_trait::async_trait;
use base::team::TeamMember;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

use super::providers;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TeamProvider {
    async fn get(&self, pars: GetTeamParsJs) -> Result<GetTeamResJs, FrError>;

    async fn add_team_member(
        &self,
        pars: AddTeamMemberParsJs,
    ) -> Result<AddTeamMemberResJs, FrError>;
    async fn edit_team_member(
        &self,
        pars: EditTeamMemberParsJs,
    ) -> Result<EditTeamMemberResJs, FrError>;

    async fn set(&self, pars: SetTeamParsJs) -> Result<SetTeamResJs, FrError>;
    async fn submit(&self, pars: SubmitSetTeamParJs) -> Result<(), FrError>;
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct GetTeamParsJs {
    pub url: String,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
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

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct GetTeamResJs {
    pub team: Vec<TeamMember>,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct AddTeamMemberParsJs {
    pub inputs: TeamMemberInputs, // directly here ok since it's just strings currently
    pub existing_members: Vec<TeamMember>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct AddTeamMemberResJs {
    pub team: Vec<TeamMember>, // display
    pub to_save: String,       // upload to IPFS
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct EditTeamMemberParsJs {
    pub inputs: TeamMember,
    pub existing_members: Vec<TeamMember>,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct EditTeamMemberResJs {
    pub team: Vec<TeamMember>, // display
    pub to_save: String,       // upload to IPFS
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SetTeamParsJs {
    pub dao_id: String,
    pub owner_address: String,
    pub url: String,
}

#[derive(Tsify, Debug, Clone, Serialize)]
#[tsify(into_wasm_abi)]
pub struct SetTeamResJs {
    pub to_sign: ToSignJs,
}

#[derive(Tsify, Debug, Clone, Deserialize)]
#[tsify(from_wasm_abi)]
pub struct SubmitSetTeamParJs {
    pub txs: Vec<SignedTxFromJs>,
}

#[wasm_bindgen(js_name=getTeam)]
pub async fn get_team(pars: GetTeamParsJs) -> Result<GetTeamResJs, FrError> {
    log_wrap_new("get_team", pars, async move |pars| {
        providers()?.team.get(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=addTeamMember)]
pub async fn add_team_member(pars: AddTeamMemberParsJs) -> Result<AddTeamMemberResJs, FrError> {
    log_wrap_new("add_team_member", pars, async move |pars| {
        providers()?.team.add_team_member(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=editTeamMember)]
pub async fn edit_team_member(pars: EditTeamMemberParsJs) -> Result<EditTeamMemberResJs, FrError> {
    log_wrap_new("edit_team_member", pars, async move |pars| {
        providers()?.team.edit_team_member(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=setTeam)]
pub async fn set_team(pars: SetTeamParsJs) -> Result<SetTeamResJs, FrError> {
    log_wrap_new("set_team", pars, async move |pars| {
        providers()?.team.set(pars).await
    })
    .await
}

#[wasm_bindgen(js_name=submitSetTeam)]
pub async fn submit_set_team(pars: SubmitSetTeamParJs) -> Result<(), FrError> {
    log_wrap_new("submit_set_team", pars, async move |pars| {
        providers()?.team.submit(pars).await
    })
    .await
}
