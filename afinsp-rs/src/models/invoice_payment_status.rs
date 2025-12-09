// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum InvoicePaymentStatus {
    #[default]
    #[serde(rename = "UNPAID")]
    Unpaid,

    #[serde(rename = "PARTIALLY_PAID")]
    PartiallyPaid,

    #[serde(rename = "PAID")]
    Paid,

    #[serde(rename = "ERRORED")]
    Errored,
}

impl fmt::Display for InvoicePaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Unpaid => "UNPAID",
            Self::PartiallyPaid => "PARTIALLY_PAID",
            Self::Paid => "PAID",
            Self::Errored => "ERRORED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for InvoicePaymentStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
