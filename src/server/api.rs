use anyhow::{Error, Result};
use core::api::json_workaround::{ProjectForUsersJson, ProjectJson};
use core::api::model::{ProjectForUsers, SavedWithdrawalRequest, WithdrawalRequestInputs};
use core::flows::create_project::logic::Programs;
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
            Err(s) => Err(Error::msg(s.to_owned())),
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
            Err(s) => Err(Error::msg(s.to_owned())),
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
            Err(s) => Err(Error::msg(s.to_owned())),
        }
    }

    pub async fn submit_withdrawal_request(
        &self,
        request: &WithdrawalRequestInputs,
    ) -> Result<SavedWithdrawalRequest> {
        log::debug!("calling api to submit withdrawal request: {:?}", request);

        // note that we're sending the rust Result "as is". might change this.
        let res: Result<SavedWithdrawalRequest, String> = reqwest::Client::new()
            .post(format!("{}/withdraw", self.url))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        match res {
            Ok(p) => Ok(p.try_into().map_err(Error::msg)?),
            Err(s) => Err(Error::msg(s.to_owned())),
        }
    }

    pub async fn load_withdrawal_requests(
        &self,
        project_id: &str,
    ) -> Result<Vec<SavedWithdrawalRequest>> {
        log::debug!(
            "calling api to load withdrawal requests for project id: {:?}",
            project_id
        );

        // note that we're sending the rust Result "as is". might change this.
        let res: Result<Vec<SavedWithdrawalRequest>, String> = reqwest::Client::new()
            .get(format!("{}/withdrawals/{}", self.url, project_id))
            .send()
            .await?
            .json()
            .await?;

        match res {
            Ok(p) => Ok(p.try_into().map_err(Error::msg)?),
            Err(s) => Err(Error::msg(s.to_owned())),
        }
    }

    // tmp hack, should be determined on chain
    pub async fn complete_withdrawal_request(&self, request_id: &String) -> Result<()> {
        log::debug!(
            "calling api to complete withdrawal request_id: {:?}",
            request_id
        );

        let res = reqwest::Client::new()
            .post(format!("{}/complete_withdrawal/{}", self.url, request_id))
            .send()
            .await?
            .status();

        match res.is_success() {
            true => Ok(()),
            false => Err(Error::msg(format!("Request error: {:?}", res))),
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
