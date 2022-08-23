use serde::Serialize;

use crate::inputs_validation::ValidationError;

pub fn to_validation_error_js(error: ValidationError) -> ValidationErrorJs {
    let type_ = match &error {
        ValidationError::Empty => "empty",
        ValidationError::MinLength { .. } => "min_length",
        ValidationError::MaxLength { .. } => "max_length",
        ValidationError::Min { .. } => "min",
        ValidationError::Max { .. } => "max",
        ValidationError::Address => "address",
        ValidationError::NotAnInteger => "not_int",
        ValidationError::NotPositive => "not_pos",
        ValidationError::NotADecimal => "not_dec",
        ValidationError::NotTimestamp => "not_timestamp",
        ValidationError::TooManyFractionalDigits { .. } => "max_fractionals",
        ValidationError::ShareCountLargerThanAvailable { .. } => "count_le_supply",
        ValidationError::Unexpected(_) => "unexpected",
        ValidationError::MustBeLessThanMaxInvestAmount => "must_be_less_max_invest",
        ValidationError::MustBeGreaterThanMinInvestAmount => "must_be_more_min_min_invest",
        ValidationError::BuyingLessSharesThanMinAmount { .. } => "buying_less_shares_than_min",
        ValidationError::BuyingMoreSharesThanMaxTotalAmount { .. } => "buying_more_shares_than_max",
        ValidationError::SharesForInvestorsGreaterThanSupply => {
            "shares_for_investors_greater_than_supply"
        }
        ValidationError::MustBeAfterNow => "mus_be_after_now",
    }
    .to_owned();

    ValidationErrorJs {
        type_,
        min_length: match &error {
            ValidationError::MinLength { min, actual } => Some(ValidationErrorMinLengthJs {
                min: min.to_owned(),
                actual: actual.to_owned(),
            }),
            _ => None,
        },
        max_length: match &error {
            ValidationError::MaxLength { max, actual } => Some(ValidationErrorMaxLengthJs {
                max: max.to_owned(),
                actual: actual.to_owned(),
            }),
            _ => None,
        },
        min: match &error {
            ValidationError::Min { min } => Some(ValidationErrorMinJs {
                min: min.to_owned(),
            }),
            ValidationError::BuyingLessSharesThanMinAmount { min } => Some(ValidationErrorMinJs {
                min: min.to_owned(),
            }),
            _ => None,
        },
        max: match &error {
            ValidationError::Max { max } => Some(ValidationErrorMaxJs {
                max: max.to_owned(),
            }),
            _ => None,
        },
        max_share_buy: match &error {
            ValidationError::BuyingMoreSharesThanMaxTotalAmount {
                max,
                currently_owned,
            } => Some(ValidationErrorMaxBuySharesJs {
                max: max.to_owned(),
                currently_owned: currently_owned.to_owned(),
            }),
            _ => None,
        },
        max_fractionals: match &error {
            ValidationError::TooManyFractionalDigits { max, actual } => {
                Some(TooManyFractionalDigitsJs {
                    max: max.to_owned(),
                    actual: actual.to_owned(),
                })
            }
            _ => None,
        },
        unexpected: match error {
            ValidationError::Unexpected(s) => Some(s),
            _ => None,
        },
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorJs {
    pub type_: String,
    pub min_length: Option<ValidationErrorMinLengthJs>,
    pub max_length: Option<ValidationErrorMaxLengthJs>,
    pub min: Option<ValidationErrorMinJs>,
    pub max: Option<ValidationErrorMaxJs>,
    pub max_share_buy: Option<ValidationErrorMaxBuySharesJs>,
    pub max_fractionals: Option<TooManyFractionalDigitsJs>,
    pub unexpected: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorMinLengthJs {
    pub min: String,
    pub actual: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorMaxLengthJs {
    pub max: String,
    pub actual: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorMinJs {
    pub min: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorMaxJs {
    pub max: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorMaxBuySharesJs {
    pub max: String,
    pub currently_owned: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TooManyFractionalDigitsJs {
    pub max: String,
    pub actual: String,
}
