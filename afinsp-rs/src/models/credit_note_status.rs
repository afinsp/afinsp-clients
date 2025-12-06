// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CreditNoteStatus {
    #[default]
    #[serde(rename = "DRAFT")]
    Draft,

    #[serde(rename = "FINALIZED")]
    Finalized,

    #[serde(rename = "VOIDED")]
    Voided,
}

impl fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Draft => "DRAFT",
            Self::Finalized => "FINALIZED",
            Self::Voided => "VOIDED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CreditNoteStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
