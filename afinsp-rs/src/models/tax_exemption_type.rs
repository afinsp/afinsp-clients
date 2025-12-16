// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum TaxExemptionType {
    #[default]
    #[serde(rename = "REVERSE_CHARGE")]
    ReverseCharge,

    #[serde(rename = "TAX_EXEMPT")]
    TaxExempt,

    #[serde(rename = "NOT_REGISTERED")]
    NotRegistered,
}

impl fmt::Display for TaxExemptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::ReverseCharge => "REVERSE_CHARGE",
            Self::TaxExempt => "TAX_EXEMPT",
            Self::NotRegistered => "NOT_REGISTERED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for TaxExemptionType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
