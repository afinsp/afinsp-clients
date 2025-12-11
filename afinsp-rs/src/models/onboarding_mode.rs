// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Onboarding mode for connected accounts
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum OnboardingMode {
    #[default]
    #[serde(rename = "express")]
    Express,

    #[serde(rename = "full")]
    Full,
}

impl fmt::Display for OnboardingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Express => "express",
            Self::Full => "full",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for OnboardingMode {
    fn encode(&self) -> String {
        self.to_string()
    }
}
