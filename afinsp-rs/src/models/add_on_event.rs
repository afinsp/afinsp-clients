// this file is @generated
use serde::{Deserialize, Serialize};

use super::{add_on_event_data::AddOnEventData, event_id::EventId, event_type::EventType};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AddOnEvent {
    #[serde(flatten)]
    pub flatten_addoneventdata: AddOnEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl AddOnEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_addoneventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
