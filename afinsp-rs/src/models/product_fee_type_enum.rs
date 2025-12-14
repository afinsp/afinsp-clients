// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ProductFeeTypeEnum {
    #[default]
    #[serde(rename = "RATE")]
    Rate,

    #[serde(rename = "SLOT")]
    Slot,

    #[serde(rename = "CAPACITY")]
    Capacity,

    #[serde(rename = "USAGE")]
    Usage,

    #[serde(rename = "EXTRA_RECURRING")]
    ExtraRecurring,

    #[serde(rename = "ONE_TIME")]
    OneTime,
}

impl fmt::Display for ProductFeeTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Rate => "RATE",
            Self::Slot => "SLOT",
            Self::Capacity => "CAPACITY",
            Self::Usage => "USAGE",
            Self::ExtraRecurring => "EXTRA_RECURRING",
            Self::OneTime => "ONE_TIME",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for ProductFeeTypeEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
