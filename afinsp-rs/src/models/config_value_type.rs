// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Authoritative value type of a Config feature. `MAP`/`JSON` both carry a JSON value.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ConfigValueType {
    #[default]
    #[serde(rename = "NUMBER")]
    Number,

    #[serde(rename = "BOOLEAN")]
    Boolean,

    #[serde(rename = "TEXT")]
    Text,

    #[serde(rename = "MAP")]
    Map,

    #[serde(rename = "JSON")]
    Json,

    #[serde(rename = "SELECT")]
    Select,
}

impl fmt::Display for ConfigValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Number => "NUMBER",
            Self::Boolean => "BOOLEAN",
            Self::Text => "TEXT",
            Self::Map => "MAP",
            Self::Json => "JSON",
            Self::Select => "SELECT",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for ConfigValueType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
