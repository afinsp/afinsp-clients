// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_id::EventId, event_type::EventType, plan_event_data::PlanEventData};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlanEvent {
    #[serde(flatten)]
    pub flatten_planeventdata: PlanEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl PlanEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_planeventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
