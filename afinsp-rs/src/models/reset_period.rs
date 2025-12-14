// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_cycle_reset_period::BillingCycleResetPeriod,
    calendar_reset_period::CalendarResetPeriod, fixed_window_reset_period::FixedWindowResetPeriod,
    never_reset_period::NeverResetPeriod, sliding_window_reset_period::SlidingWindowResetPeriod,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ResetPeriod {
    #[serde(rename = "BILLING_CYCLE")]
    BillingCycle(BillingCycleResetPeriod),
    #[serde(rename = "CALENDAR")]
    Calendar(CalendarResetPeriod),
    #[serde(rename = "FIXED_WINDOW")]
    FixedWindow(FixedWindowResetPeriod),
    #[serde(rename = "SLIDING_WINDOW")]
    SlidingWindow(SlidingWindowResetPeriod),
    #[serde(rename = "NEVER")]
    Never(NeverResetPeriod),
}

impl Default for ResetPeriod {
    fn default() -> Self {
        Self::BillingCycle(BillingCycleResetPeriod::default())
    }
}
