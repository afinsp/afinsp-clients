// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CancelSubscriptionRequest {
    /// If not provided, the cancellation will be effective at the end of the current billing or committed period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CancelSubscriptionRequest {
    pub fn new() -> Self {
        Self {
            effective_date: None,
            reason: None,
        }
    }
}
