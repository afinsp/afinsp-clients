// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PaymentTypeEnum {
    #[default]
    #[serde(rename = "PAYMENT")]
    Payment,

    #[serde(rename = "REFUND")]
    Refund,
}

impl fmt::Display for PaymentTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Payment => "PAYMENT",
            Self::Refund => "REFUND",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for PaymentTypeEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
