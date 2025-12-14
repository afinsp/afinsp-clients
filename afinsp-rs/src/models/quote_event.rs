// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_id::EventId, event_type::EventType, quote_event_data::QuoteEventData};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct QuoteEvent {
    #[serde(flatten)]
    pub flatten_quoteeventdata: QuoteEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl QuoteEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_quoteeventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
