// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CheckoutSessionStatus {
    #[default]
    #[serde(rename = "CREATED")]
    Created,

    #[serde(rename = "AWAITING_PAYMENT")]
    AwaitingPayment,

    #[serde(rename = "COMPLETED")]
    Completed,

    #[serde(rename = "EXPIRED")]
    Expired,

    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl fmt::Display for CheckoutSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Created => "CREATED",
            Self::AwaitingPayment => "AWAITING_PAYMENT",
            Self::Completed => "COMPLETED",
            Self::Expired => "EXPIRED",
            Self::Cancelled => "CANCELLED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CheckoutSessionStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
