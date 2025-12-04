// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CalendarUnit {
    #[default]
    #[serde(rename = "HOUR")]
    Hour,

    #[serde(rename = "DAY")]
    Day,

    #[serde(rename = "WEEK")]
    Week,

    #[serde(rename = "MONTH")]
    Month,

    #[serde(rename = "YEAR")]
    Year,
}

impl fmt::Display for CalendarUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Hour => "HOUR",
            Self::Day => "DAY",
            Self::Week => "WEEK",
            Self::Month => "MONTH",
            Self::Year => "YEAR",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CalendarUnit {
    fn encode(&self) -> String {
        self.to_string()
    }
}
