// this file is @generated
use serde::{Deserialize, Serialize};

use super::ingest_failure::IngestFailure;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEventsResponse {
    /// Events that failed to ingest. Omitted when no failures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<IngestFailure>>,
}

impl IngestEventsResponse {
    pub fn new() -> Self {
        Self { failures: None }
    }
}
