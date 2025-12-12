// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum PaymentStatusEnum {
    #[default]
    #[serde(rename = "READY")]
    Ready,

    #[serde(rename = "PENDING")]
    Pending,

    #[serde(rename = "SETTLED")]
    Settled,

    #[serde(rename = "CANCELLED")]
    Cancelled,

    #[serde(rename = "FAILED")]
    Failed,
}

impl fmt::Display for PaymentStatusEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Ready => "READY",
            Self::Pending => "PENDING",
            Self::Settled => "SETTLED",
            Self::Cancelled => "CANCELLED",
            Self::Failed => "FAILED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for PaymentStatusEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
