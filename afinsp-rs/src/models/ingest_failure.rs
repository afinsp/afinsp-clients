// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestFailure {
    pub event_id: String,

    pub reason: String,
}

impl IngestFailure {
    pub fn new(event_id: String, reason: String) -> Self {
        Self { event_id, reason }
    }
}
