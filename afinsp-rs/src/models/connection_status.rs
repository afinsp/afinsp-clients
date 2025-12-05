// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Status of a connected account
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ConnectionStatus {
    #[default]
    #[serde(rename = "pending")]
    Pending,

    #[serde(rename = "active")]
    Active,

    #[serde(rename = "revoked")]
    Revoked,

    #[serde(rename = "suspended")]
    Suspended,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::Suspended => "suspended",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for ConnectionStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
