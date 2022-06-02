use crate::teal::{customer_escrow, dao_app_approval, dao_app_clear};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use base::teal::TealApi;
use mbase::{
    api::contract::Contract,
    api::version::{Version, VersionedTealSourceTemplate, Versions},
    teal::TealSourceTemplate,
};

pub struct TealStringsApi {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl TealApi for TealStringsApi {
    async fn last_versions(&self) -> Result<Versions> {
        Ok(Versions {
            app_approval: Version(1),
            app_clear: Version(1),
            customer_escrow: Version(1),
        })
    }

    async fn template(
        &self,
        contract: Contract,
        version: Version,
    ) -> Result<VersionedTealSourceTemplate> {
        match contract {
            Contract::DaoCustomer => dao_customer_teal(version),
            Contract::DaoAppApproval => dao_app_approval_teal(version),
            Contract::DaoAppClear => dao_app_clear_teal(version),
        }
    }
}

fn dao_customer_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, customer_escrow::SRC),
        _ => not_found_err("dao customer", version),
    }
}

fn dao_app_approval_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, dao_app_approval::SRC),
        _ => not_found_err("dao app", version),
    }
}

fn dao_app_clear_teal(version: Version) -> Result<VersionedTealSourceTemplate> {
    match version.0 {
        1 => load_versioned_teal_template(version, dao_app_clear::SRC),
        _ => not_found_err("dao app", version),
    }
}

fn load_versioned_teal_template(
    version: Version,
    str: &str,
) -> Result<VersionedTealSourceTemplate> {
    Ok(VersionedTealSourceTemplate {
        version,
        template: TealSourceTemplate(str.as_bytes().to_vec()),
    })
}

fn not_found_err<T>(id: &str, version: Version) -> Result<T> {
    Err(anyhow!("Not found version: {version:?} for: {id}"))
}
