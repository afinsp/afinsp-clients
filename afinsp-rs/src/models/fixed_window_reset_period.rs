// this file is @generated
use serde::{Deserialize, Serialize};

use super::calendar_unit::CalendarUnit;

/// Resets at regular intervals — anchored to your subscription's exact activation time.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FixedWindowResetPeriod {
    pub interval: i32,

    pub unit: CalendarUnit,
}

impl FixedWindowResetPeriod {
    pub fn new(interval: i32, unit: CalendarUnit) -> Self {
        Self { interval, unit }
    }
}
