// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    capacity_plan_fee::CapacityPlanFee, extra_recurring_plan_fee::ExtraRecurringPlanFee,
    one_time_plan_fee::OneTimePlanFee, rate_plan_fee::RatePlanFee, slot_plan_fee::SlotPlanFee,
    usage_plan_fee::UsagePlanFee,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Fee {
    #[serde(rename = "RATE")]
    Rate(RatePlanFee),
    #[serde(rename = "SLOT")]
    Slot(SlotPlanFee),
    #[serde(rename = "CAPACITY")]
    Capacity(CapacityPlanFee),
    #[serde(rename = "USAGE")]
    Usage(UsagePlanFee),
    #[serde(rename = "EXTRA_RECURRING")]
    ExtraRecurring(ExtraRecurringPlanFee),
    #[serde(rename = "ONE_TIME")]
    OneTime(OneTimePlanFee),
}

impl Default for Fee {
    fn default() -> Self {
        Self::Rate(RatePlanFee::default())
    }
}
