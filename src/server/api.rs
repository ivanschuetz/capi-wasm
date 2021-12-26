use anyhow::{Error, Result};
use core::api::json_workaround::{ProjectForUsersJson, ProjectJson};
use core::api::model::ProjectForUsers;
use core::flows::create_project::create_project::Programs;
use core::flows::create_project::model::Project;
use core::teal::{TealSource, TealSourceTemplate};
use std::convert::TryInto;

use super::mock::teal::{
    central_app_approve, central_app_clear, central_escrow, customer_escrow, invest_escrow,
    staking_escrow,
};

// ideally a trait, but async trait not supported by rust yet
// error msg recommends using async-trait lib, passing for now
// (not adding more deps to wasm than strictly needed: compatibility / binary size)
pub struct Api {
    url: String, // req
}

impl Api {
    pub fn new(url: String) -> Api {
        Api { url }
    }

    pub async fn save_project(&self, project: &Project) -> Result<ProjectForUsers> {
        let project: ProjectJson = project.to_owned().into();
        log::debug!("calling api to save project: {:?}", project);

        // note that we're sending the rust Result "as is". might change this.
        let res: Result<ProjectForUsersJson, String> = reqwest::Client::new()
            .post(format!("{}/save", self.url))
            .json(&project)
            .send()
            .await?
            .json()
            .await?;

        match res {
            Ok(p) => Ok(p.try_into().map_err(Error::msg)?),
            Err(s) => Err(Error::msg(s)),
        }
    }

    pub async fn load_project_user_view(&self, id: &str) -> Result<ProjectForUsers> {
        log::debug!("calling api to load project with id: {:?}", id);

        // note that we're sending the rust Result "as is". might change this.
        let res: Result<ProjectForUsersJson, String> = reqwest::Client::new()
            .get(format!("{}/invest/{}", self.url, id))
            .send()
            .await?
            .json()
            .await?;

        match res {
            Ok(p) => Ok(p.try_into().map_err(Error::msg)?),
            Err(s) => Err(Error::msg(s)),
        }
    }

    pub async fn load_project(&self, id: &str) -> Result<Project> {
        log::debug!("calling api to load project with id: {:?}", id);

        // note that we're sending the rust Result "as is". might change this.
        let res: Result<ProjectJson, String> = reqwest::Client::new()
            .get(format!("{}/project/{}", self.url, id))
            .send()
            .await?
            .json()
            .await?;

        match res {
            Ok(p) => Ok(p.try_into().map_err(Error::msg)?),
            Err(s) => Err(Error::msg(s)),
        }
    }
}

pub fn programs() -> Result<Programs> {
    Ok(Programs {
        central_app_approval: TealSourceTemplate(central_app_approve::SRC.as_bytes().to_vec()),
        central_app_clear: TealSource(central_app_clear::SRC.as_bytes().to_vec()),
        central_escrow: TealSourceTemplate(central_escrow::SRC.as_bytes().to_vec()),
        customer_escrow: TealSourceTemplate(customer_escrow::SRC.as_bytes().to_vec()),
        invest_escrow: TealSourceTemplate(invest_escrow::SRC.as_bytes().to_vec()),
        staking_escrow: TealSourceTemplate(staking_escrow::SRC.as_bytes().to_vec()),
    })
}
