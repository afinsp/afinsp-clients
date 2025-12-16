// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    capacity_fee::CapacityFee, one_time_fee::OneTimeFee, rate_fee::RateFee,
    recurring_fee::RecurringFee, slot_fee::SlotFee, usage_fee::UsageFee,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SubscriptionFee {
    #[serde(rename = "RATE")]
    Rate(RateFee),
    #[serde(rename = "ONE_TIME")]
    OneTime(OneTimeFee),
    #[serde(rename = "RECURRING")]
    Recurring(RecurringFee),
    #[serde(rename = "CAPACITY")]
    Capacity(CapacityFee),
    #[serde(rename = "SLOT")]
    Slot(SlotFee),
    #[serde(rename = "USAGE")]
    Usage(UsageFee),
}

impl Default for SubscriptionFee {
    fn default() -> Self {
        Self::Rate(RateFee::default())
    }
}
