use serde::Serialize;

use crate::js::create_dao::create_dao::ValidateDaoInputsError;

/// Note String used for many originally numeric fields: these fields are only to display to the user
/// using String allows to reuse them easily for different numbers, like u64 or Decimal and format them
#[derive(Debug, Clone, Serialize)]
pub enum ValidationError {
    Empty,
    MinLength {
        min: String,
        actual: String,
    },
    MaxLength {
        max: String,
        actual: String,
    },
    Min {
        min: String,
        actual: String,
    },
    Max {
        max: String,
        actual: String,
    },
    Address,
    NotAnInteger,
    NotADecimal,
    TooManyFractionalDigits {
        max: String,
        actual: String,
    },
    /// Related to validation but not directly attributable to the user (e.g. overflows when converting entered quantities to base units).
    /// Shouldn't happen normally - the conditions leading to these errors should be validated.
    Unexpected(String),
}

/// Temporary hack for backwards compatibility with previous validation (which returned only a string)
/// TODO all places that can trigger ValidationError should be adjusted in JS to handle the structured validation errors
impl From<ValidationError> for anyhow::Error {
    fn from(error: ValidationError) -> Self {
        anyhow::Error::msg(format!("{error:?}"))
    }
}

/// Temporary hack for backwards compatibility with previous validation (which returned only a string)
/// TODO all places that can trigger ValidationError should be adjusted in JS to handle the structured validation errors
impl From<ValidateDaoInputsError> for anyhow::Error {
    fn from(error: ValidateDaoInputsError) -> Self {
        anyhow::Error::msg(format!("{error:?}"))
    }
}
