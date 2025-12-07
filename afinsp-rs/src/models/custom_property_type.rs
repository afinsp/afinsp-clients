// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CustomPropertyType {
    #[default]
    #[serde(rename = "TEXT")]
    Text,

    #[serde(rename = "NUMBER")]
    Number,

    #[serde(rename = "BOOLEAN")]
    Boolean,

    #[serde(rename = "DATE")]
    Date,

    #[serde(rename = "DATETIME")]
    Datetime,

    #[serde(rename = "SINGLE_SELECT")]
    SingleSelect,

    #[serde(rename = "MULTI_SELECT")]
    MultiSelect,

    #[serde(rename = "JSON")]
    Json,

    #[serde(rename = "URL")]
    Url,

    #[serde(rename = "EMAIL")]
    Email,
}

impl fmt::Display for CustomPropertyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Text => "TEXT",
            Self::Number => "NUMBER",
            Self::Boolean => "BOOLEAN",
            Self::Date => "DATE",
            Self::Datetime => "DATETIME",
            Self::SingleSelect => "SINGLE_SELECT",
            Self::MultiSelect => "MULTI_SELECT",
            Self::Json => "JSON",
            Self::Url => "URL",
            Self::Email => "EMAIL",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CustomPropertyType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
