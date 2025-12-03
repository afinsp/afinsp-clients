// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum BatchJobStatus {
    #[default]
    #[serde(rename = "PENDING")]
    Pending,

    #[serde(rename = "CHUNKING")]
    Chunking,

    #[serde(rename = "PROCESSING")]
    Processing,

    #[serde(rename = "COMPLETED")]
    Completed,

    #[serde(rename = "COMPLETED_WITH_ERRORS")]
    CompletedWithErrors,

    #[serde(rename = "FAILED")]
    Failed,

    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl fmt::Display for BatchJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Pending => "PENDING",
            Self::Chunking => "CHUNKING",
            Self::Processing => "PROCESSING",
            Self::Completed => "COMPLETED",
            Self::CompletedWithErrors => "COMPLETED_WITH_ERRORS",
            Self::Failed => "FAILED",
            Self::Cancelled => "CANCELLED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for BatchJobStatus {
    fn encode(&self) -> String {
        self.to_string()
    }
}
