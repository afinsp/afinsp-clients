// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    capacity_pricing::CapacityPricing, extra_recurring_pricing::ExtraRecurringPricing,
    one_time_pricing::OneTimePricing, rate_pricing::RatePricing, slot_pricing::SlotPricing,
    usage_pricing::UsagePricing,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Pricing {
    #[serde(rename = "RATE")]
    Rate(RatePricing),
    #[serde(rename = "SLOT")]
    Slot(SlotPricing),
    #[serde(rename = "CAPACITY")]
    Capacity(CapacityPricing),
    #[serde(rename = "USAGE")]
    Usage(UsagePricing),
    #[serde(rename = "EXTRA_RECURRING")]
    ExtraRecurring(ExtraRecurringPricing),
    #[serde(rename = "ONE_TIME")]
    OneTime(OneTimePricing),
}

impl Default for Pricing {
    fn default() -> Self {
        Self::Rate(RatePricing::default())
    }
}
