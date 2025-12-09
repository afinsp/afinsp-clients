// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Lifecycle status of a feature.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum FeatureStatus {
    #[default]
    #[serde(rename = "ACTIVE")]
    Active,

    #[serde(rename = "DISABLED")]
    Disabled,

    #[serde(rename = "ARCHIVED")]
    Archived,
}

impl fmt::Display for FeatureStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Active => "ACTIVE",
            Self::Disabled => "DISABLED",
            Self::Archived => "ARCHIVED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for FeatureStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
