// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer_id::CustomerId, quote_id::QuoteId, subscription_id::SubscriptionId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct QuoteEventData {
    pub customer_id: CustomerId,

    pub quote_id: QuoteId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,
}

impl QuoteEventData {
    pub fn new(customer_id: CustomerId, quote_id: QuoteId) -> Self {
        Self {
            customer_id,
            quote_id,
            subscription_id: None,
        }
    }
}
