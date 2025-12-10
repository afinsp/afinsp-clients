// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_id::EventId, event_type::EventType, metric_event_data::MetricEventData};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetricEvent {
    #[serde(flatten)]
    pub flatten_metriceventdata: MetricEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl MetricEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_metriceventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
