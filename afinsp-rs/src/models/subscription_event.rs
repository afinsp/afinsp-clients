// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    event_id::EventId, event_type::EventType, subscription_event_data::SubscriptionEventData,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionEvent {
    #[serde(flatten)]
    pub flatten_subscriptioneventdata: SubscriptionEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl SubscriptionEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_subscriptioneventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
