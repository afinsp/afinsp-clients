// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum InvoiceStatus {
    #[default]
    #[serde(rename = "DRAFT")]
    Draft,

    #[serde(rename = "FINALIZED")]
    Finalized,

    #[serde(rename = "UNCOLLECTIBLE")]
    Uncollectible,

    #[serde(rename = "VOID")]
    Void,
}

impl fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Draft => "DRAFT",
            Self::Finalized => "FINALIZED",
            Self::Uncollectible => "UNCOLLECTIBLE",
            Self::Void => "VOID",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for InvoiceStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
