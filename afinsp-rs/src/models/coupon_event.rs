// this file is @generated
use serde::{Deserialize, Serialize};

use super::{coupon_event_data::CouponEventData, event_id::EventId, event_type::EventType};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CouponEvent {
    #[serde(flatten)]
    pub flatten_couponeventdata: CouponEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl CouponEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_couponeventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
