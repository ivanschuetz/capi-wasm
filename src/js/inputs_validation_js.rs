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
        ValidationError::CompressedImageSize { .. } => "max_img_size",
        ValidationError::Unexpected(_) => "unexpected",
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
            ValidationError::Min { min, actual } => Some(ValidationErrorMinJs {
                min: min.to_owned(),
                actual: actual.to_owned(),
            }),
            _ => None,
        },
        max: match &error {
            ValidationError::Max { max, actual } => Some(ValidationErrorMaxJs {
                max: max.to_owned(),
                actual: actual.to_owned(),
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
    pub actual: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationErrorMaxJs {
    pub max: String,
    pub actual: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TooManyFractionalDigitsJs {
    pub max: String,
    pub actual: String,
}
