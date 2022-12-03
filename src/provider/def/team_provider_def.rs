use crate::error::FrError;
use crate::inputs_validation::ValidationError;
use crate::js::common::signed_js_tx_to_signed_tx1;
use crate::js::to_sign_js::ToSignJs;
use crate::provider::create_dao_provider::{
    validate_text_min_max_length, validate_url, validate_url_opt,
};
use crate::provider::team_provider::{
    AddTeamMemberParsJs, AddTeamMemberResJs, EditTeamMemberParsJs, EditTeamMemberResJs,
    GetTeamParsJs, GetTeamResJs, SetTeamParsJs, SetTeamResJs, SubmitSetTeamParJs, TeamMemberInputs,
    TeamMemberJs, TeamProvider,
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::api::fetcher::Fetcher;
use base::dependencies::fetcher;
use base::dev_settings::{submit_dev_settings, DevSettingsSigned};
use base::team::{team, TeamMember};
use mbase::dependencies::algod;
use mbase::util::network_util::wait_for_pending_transaction;
use serde::Serialize;
use tsify::Tsify;
use uuid::Uuid;

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

    let validated_inputs = validate_team_member_inputs(&pars.inputs)?;

    members.push(validated_inputs.to_team_member().into());

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

fn validate_team_member_inputs(
    inputs: &TeamMemberInputs,
) -> Result<ValidatedTeamMemberInputs, ValidateTeamMemberInputsError> {
    let name_res = validate_name(&inputs.name);
    let descr_res = validate_descr(&inputs.descr);
    let role_res = validate_role(&inputs.role);
    let picture_res = validate_url(&inputs.picture);
    let github_url_res = validate_github_url(&inputs.github_link);
    let twitter_url_res = validate_twitter_url(&inputs.twitter_link);
    let linkedin_url_res = validate_linkedin_url(&inputs.linkedin_link);

    match (
        &name_res,
        &descr_res,
        &role_res,
        &picture_res,
        &github_url_res,
        &twitter_url_res,
        &linkedin_url_res,
    ) {
        (
            Ok(name),
            Ok(descr),
            Ok(role),
            Ok(picture),
            Ok(github_url),
            Ok(twitter_url),
            Ok(linkedin_url),
        ) => Ok(ValidatedTeamMemberInputs {
            name: name.clone(),
            descr: descr.clone(),
            role: role.clone(),
            picture: picture.clone(),
            github_url: github_url.clone(),
            twitter_url: twitter_url.clone(),
            linkedin_url: linkedin_url.clone(),
        }),
        _ => Err(ValidateTeamMemberInputsError::AllFieldsValidation(
            AddTeamMemberInputErrors {
                name: name_res.err(),
                descr: descr_res.err(),
                role: role_res.err(),
                picture: picture_res.err(),
                github_url: github_url_res.err(),
                twitter_url: twitter_url_res.err(),
                linkedin_url: linkedin_url_res.err(),
            },
        )),
    }
}

struct ValidatedTeamMemberInputs {
    name: String,
    descr: String,
    role: String,
    picture: String,
    github_url: Option<String>,
    twitter_url: Option<String>,
    linkedin_url: Option<String>,
}

impl ValidatedTeamMemberInputs {
    pub fn to_team_member(&self) -> TeamMember {
        TeamMember {
            uuid: Uuid::new_v4().to_string(),
            name: self.name.clone(),
            descr: self.descr.clone(),
            role: self.role.clone(),
            picture: self.picture.clone(),
            github_link: self.github_url.clone(),
            twitter_link: self.twitter_url.clone(),
            linkedin_link: self.linkedin_url.clone(),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize)]
pub enum ValidateTeamMemberInputsError {
    AllFieldsValidation(AddTeamMemberInputErrors),
    NonValidation(String),
}
/// Errors to be shown next to the respective input fields
#[derive(Tsify, Debug, Clone, Serialize, Default)]
#[tsify(into_wasm_abi)]
pub struct AddTeamMemberInputErrors {
    pub name: Option<ValidationError>,
    pub descr: Option<ValidationError>,
    pub role: Option<ValidationError>,
    pub picture: Option<ValidationError>,
    pub github_url: Option<ValidationError>,
    pub twitter_url: Option<ValidationError>,
    pub linkedin_url: Option<ValidationError>,
}

fn validate_name(name: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(name, 0, 200)
}

fn validate_descr(descr: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(descr, 0, 400)
}

fn validate_role(role: &str) -> Result<String, ValidationError> {
    validate_text_min_max_length(role, 0, 400)
}

fn validate_github_url(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    validate_url_opt(url)
}

fn validate_twitter_url(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    validate_url_opt(url)
}

fn validate_linkedin_url(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    validate_url_opt(url)
}

fn validate_image_url(url: &Option<String>) -> Result<Option<String>, ValidationError> {
    validate_url_opt(url)
}
