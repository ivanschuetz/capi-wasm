use algonaut::{algod::v2::Algod, core::MicroAlgos};
use anyhow::Result;
use make::{
    api::model::ProjectForUsers,
    flows::withdraw::logic::{FIXED_FEE, MIN_BALANCE},
};

pub async fn available_funds(algod: &Algod, project: &ProjectForUsers) -> Result<MicroAlgos> {
    let customer_escrow_balance = algod
        .account_information(&project.customer_escrow_address)
        .await?
        .amount;

    let central_escrow_balance = algod
        .account_information(&project.central_escrow_address)
        .await?
        .amount;

    // TODO dynamic MIN_BALANCE? (and fee)
    Ok(customer_escrow_balance + central_escrow_balance - (MIN_BALANCE + FIXED_FEE) * 2)
}
