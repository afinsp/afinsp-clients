// this file is @generated
use serde::{Deserialize, Serialize};

use super::{event_id::EventId, event_type::EventType, invoice_event_data::InvoiceEventData};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InvoiceEvent {
    #[serde(flatten)]
    pub flatten_invoiceeventdata: InvoiceEventData,

    pub id: EventId,

    pub timestamp: String,

    pub r#type: EventType,
}

impl InvoiceEvent {
    pub fn new(id: EventId, timestamp: String, r#type: EventType) -> Self {
        Self {
            flatten_invoiceeventdata: Default::default(),
            id,
            timestamp,
            r#type,
        }
    }
}
