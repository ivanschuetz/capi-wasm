use algonaut::{core::CompiledTeal, transaction::contract_account::ContractAccount};
use mbase::api::version::{Version, VersionedContractAccount};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::error::Error;

type DefaultError = Box<dyn Error + Send + Sync>;

/////////////////////////////////////////////////////////////////////////////////////////////////
// workaround for some algonaut types not being serializable with json (only msg pack)
// we could serialize them with msg pack but for now json is better for debugging
// (e.g. web proxy, or in js for the wasm interface)
/////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAccountJs {
    pub address: String,
    pub program: CompiledTeal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedContractAccountJs {
    pub version: String,
    pub contract: ContractAccountJs,
}

impl From<ContractAccount> for ContractAccountJs {
    fn from(ca: ContractAccount) -> Self {
        ContractAccountJs {
            address: ca.address().to_string(),
            program: ca.program,
        }
    }
}

impl From<VersionedContractAccount> for VersionedContractAccountJs {
    fn from(ca: VersionedContractAccount) -> Self {
        VersionedContractAccountJs {
            version: ca.version.0.to_string(),
            contract: ca.account.into(),
        }
    }
}

impl TryFrom<ContractAccountJs> for ContractAccount {
    type Error = DefaultError;

    fn try_from(ca: ContractAccountJs) -> Result<Self, Self::Error> {
        let account = ContractAccount::new(ca.program);

        // ContractAccount calculates the hash (address) - just double checking that the address we're discarding is the same
        if account.address().to_string() != ca.address {
            return Err(format!(
                "Invalid state: the address: {} doesn't correspond to the program: {:?}",
                ca.address, account.program
            )
            .into());
        }

        Ok(account)
    }
}

impl TryFrom<VersionedContractAccountJs> for VersionedContractAccount {
    type Error = DefaultError;

    fn try_from(ca: VersionedContractAccountJs) -> Result<Self, Self::Error> {
        let version = Version(ca.version.parse()?);
        let contract = ca.contract.try_into()?;

        Ok(VersionedContractAccount {
            version,
            account: contract,
        })
    }
}
