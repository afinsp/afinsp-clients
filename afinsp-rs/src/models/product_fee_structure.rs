// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    capacity_fee_structure::CapacityFeeStructure,
    extra_recurring_fee_structure::ExtraRecurringFeeStructure,
    one_time_fee_structure::OneTimeFeeStructure, rate_fee_structure::RateFeeStructure,
    slot_fee_structure::SlotFeeStructure, usage_fee_structure::UsageFeeStructure,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ProductFeeStructure {
    #[serde(rename = "RATE")]
    Rate(RateFeeStructure),
    #[serde(rename = "SLOT")]
    Slot(SlotFeeStructure),
    #[serde(rename = "CAPACITY")]
    Capacity(CapacityFeeStructure),
    #[serde(rename = "USAGE")]
    Usage(UsageFeeStructure),
    #[serde(rename = "EXTRA_RECURRING")]
    ExtraRecurring(ExtraRecurringFeeStructure),
    #[serde(rename = "ONE_TIME")]
    OneTime(OneTimeFeeStructure),
}

impl Default for ProductFeeStructure {
    fn default() -> Self {
        Self::Rate(RateFeeStructure::default())
    }
}
