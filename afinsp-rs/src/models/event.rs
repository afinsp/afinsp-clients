// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Event {
    /// Billable metric code. Max 512 characters.
    pub code: String,

    /// Afinsp customer ID or external customer alias.
    pub customer_id: String,

    /// Unique event identifier. Max 255 characters. A UUID or ULID is recommended.
    pub event_id: String,

    /// Arbitrary string key-value pairs used by billable metrics for filtering and aggregation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,

    /// RFC 3339 timestamp. Defaults to ingestion time if omitted.
    /// Must be between 24 hours ago and 1 hour from now. Set `allow_backfilling` to remove the past limit.
    pub timestamp: String,
}

impl Event {
    pub fn new(code: String, customer_id: String, event_id: String, timestamp: String) -> Self {
        Self {
            code,
            customer_id,
            event_id,
            properties: None,
            timestamp,
        }
    }
}
