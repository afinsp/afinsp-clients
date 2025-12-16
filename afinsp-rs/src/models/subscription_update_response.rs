// this file is @generated
use serde::{Deserialize, Serialize};

use super::subscription_details::SubscriptionDetails;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionUpdateResponse {
    pub subscription: SubscriptionDetails,
}

impl SubscriptionUpdateResponse {
    pub fn new(subscription: SubscriptionDetails) -> Self {
        Self { subscription }
    }
}
