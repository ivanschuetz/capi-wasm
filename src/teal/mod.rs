mod app_central_approval;
mod app_central_clear;
mod central_escrow;
mod customer_escrow;
mod investing_escrow;
mod locking_escrow;
mod update_teal;

use core::flows::create_dao::create_dao::{Escrows, Programs};
use core::teal::{TealSource, TealSourceTemplate};

pub fn programs() -> Programs {
    Programs {
        central_app_approval: TealSourceTemplate(app_central_approval::SRC.as_bytes().to_vec()),
        central_app_clear: TealSource(app_central_clear::SRC.as_bytes().to_vec()),
        escrows: Escrows {
            central_escrow: TealSourceTemplate(central_escrow::SRC.as_bytes().to_vec()),
            customer_escrow: TealSourceTemplate(customer_escrow::SRC.as_bytes().to_vec()),
            invest_escrow: TealSourceTemplate(investing_escrow::SRC.as_bytes().to_vec()),
            locking_escrow: TealSourceTemplate(locking_escrow::SRC.as_bytes().to_vec()),
        },
    }
}
