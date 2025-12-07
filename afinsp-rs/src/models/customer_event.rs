// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer_event_data::CustomerEventData, event_id::EventId, event_type::EventType};

/// Event-specific webhook schemas for type-safe webhook payloads
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerEvent {
    #[serde(flatten)]
    pub flatten_customereventdata: CustomerEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl CustomerEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_customereventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
