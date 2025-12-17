// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum UsageModelEnum {
    #[default]
    #[serde(rename = "PER_UNIT")]
    PerUnit,

    #[serde(rename = "TIERED")]
    Tiered,

    #[serde(rename = "VOLUME")]
    Volume,

    #[serde(rename = "PACKAGE")]
    Package,

    #[serde(rename = "MATRIX")]
    Matrix,
}

impl fmt::Display for UsageModelEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::PerUnit => "PER_UNIT",
            Self::Tiered => "TIERED",
            Self::Volume => "VOLUME",
            Self::Package => "PACKAGE",
            Self::Matrix => "MATRIX",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for UsageModelEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
