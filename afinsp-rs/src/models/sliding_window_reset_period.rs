// this file is @generated
use serde::{Deserialize, Serialize};

use super::calendar_unit::CalendarUnit;

/// Always ends at now — e.g. 30 days means the last 30 days, old usage drops off automatically.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SlidingWindowResetPeriod {
    pub interval: i32,

    pub unit: CalendarUnit,
}

impl SlidingWindowResetPeriod {
    pub fn new(interval: i32, unit: CalendarUnit) -> Self {
        Self { interval, unit }
    }
}
