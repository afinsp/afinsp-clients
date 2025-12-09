// this file is @generated
use serde::{Deserialize, Serialize};

use super::event::Event;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEventsRequest {
    /// Allow events with timestamps more than 1 day in the past. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_backfilling: Option<bool>,

    /// Accept the batch even if some events fail validation. Defaults to `false`.
    /// When `true`, valid events are ingested and failures are reported in the response body.
    /// When `false` (default), any invalid event rejects the entire batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partial_failures: Option<bool>,

    /// 1–100 events per request.
    pub events: Vec<Event>,
}

impl IngestEventsRequest {
    pub fn new(events: Vec<Event>) -> Self {
        Self {
            allow_backfilling: None,
            allow_partial_failures: None,
            events,
        }
    }
}
