// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CreditType {
    #[default]
    #[serde(rename = "CREDIT_TO_BALANCE")]
    CreditToBalance,

    #[serde(rename = "REFUND")]
    Refund,

    #[serde(rename = "DEBT_CANCELLATION")]
    DebtCancellation,
}

impl fmt::Display for CreditType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::CreditToBalance => "CREDIT_TO_BALANCE",
            Self::Refund => "REFUND",
            Self::DebtCancellation => "DEBT_CANCELLATION",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CreditType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
