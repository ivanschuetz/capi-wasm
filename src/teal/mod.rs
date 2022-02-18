mod central_app_approve;
mod central_app_clear;
mod central_escrow;
mod customer_escrow;
mod invest_escrow;
mod locking_escrow;

use core::flows::create_project::create_project::{Escrows, Programs};
use core::teal::{TealSource, TealSourceTemplate};

pub fn programs() -> Programs {
    Programs {
        central_app_approval: TealSourceTemplate(central_app_approve::SRC.as_bytes().to_vec()),
        central_app_clear: TealSource(central_app_clear::SRC.as_bytes().to_vec()),
        escrows: Escrows {
            central_escrow: TealSourceTemplate(central_escrow::SRC.as_bytes().to_vec()),
            customer_escrow: TealSourceTemplate(customer_escrow::SRC.as_bytes().to_vec()),
            invest_escrow: TealSourceTemplate(invest_escrow::SRC.as_bytes().to_vec()),
            locking_escrow: TealSourceTemplate(locking_escrow::SRC.as_bytes().to_vec()),
        },
    }
}
