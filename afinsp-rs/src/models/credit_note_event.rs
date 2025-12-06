// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_note_event_data::CreditNoteEventData, event_id::EventId, event_type::EventType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreditNoteEvent {
    #[serde(flatten)]
    pub flatten_creditnoteeventdata: CreditNoteEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl CreditNoteEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_creditnoteeventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
