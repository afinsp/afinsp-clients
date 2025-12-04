// this file is @generated
use serde::{Deserialize, Serialize};

use super::subscription::Subscription;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CancelSubscriptionResponse {
    pub subscription: Subscription,
}

impl CancelSubscriptionResponse {
    pub fn new(subscription: Subscription) -> Self {
        Self { subscription }
    }
}
