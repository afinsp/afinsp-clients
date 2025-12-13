// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_id::EventId, event_type::EventType, product_event_data::ProductEventData};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductEvent {
    #[serde(flatten)]
    pub flatten_producteventdata: ProductEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl ProductEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_producteventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
