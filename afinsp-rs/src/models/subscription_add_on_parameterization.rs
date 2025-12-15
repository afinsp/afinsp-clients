// this file is @generated
use serde::{Deserialize, Serialize};

use super::billing_period_enum::BillingPeriodEnum;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionAddOnParameterization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<BillingPeriodEnum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub committed_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_slot_count: Option<i32>,
}

impl SubscriptionAddOnParameterization {
    pub fn new() -> Self {
        Self {
            billing_period: None,
            committed_capacity: None,
            initial_slot_count: None,
        }
    }
}
