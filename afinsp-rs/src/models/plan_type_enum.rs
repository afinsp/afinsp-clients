// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PlanTypeEnum {
    #[default]
    #[serde(rename = "STANDARD")]
    Standard,

    #[serde(rename = "FREE")]
    Free,

    #[serde(rename = "CUSTOM")]
    Custom,
}

impl fmt::Display for PlanTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Standard => "STANDARD",
            Self::Free => "FREE",
            Self::Custom => "CUSTOM",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for PlanTypeEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
