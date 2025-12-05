// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Type of connection between platform and connected account
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ConnectionType {
    #[default]
    #[serde(rename = "standard")]
    Standard,

    #[serde(rename = "express")]
    Express,
}

impl fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Standard => "standard",
            Self::Express => "express",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for ConnectionType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
