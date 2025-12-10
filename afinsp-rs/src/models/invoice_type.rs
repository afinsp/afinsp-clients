// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum InvoiceType {
    #[default]
    #[serde(rename = "RECURRING")]
    Recurring,

    #[serde(rename = "ONE_OFF")]
    OneOff,

    #[serde(rename = "ADJUSTMENT")]
    Adjustment,

    #[serde(rename = "USAGE_THRESHOLD")]
    UsageThreshold,
}

impl fmt::Display for InvoiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Recurring => "RECURRING",
            Self::OneOff => "ONE_OFF",
            Self::Adjustment => "ADJUSTMENT",
            Self::UsageThreshold => "USAGE_THRESHOLD",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for InvoiceType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
