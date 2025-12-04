// this file is @generated
use serde::{Deserialize, Serialize};

use super::calendar_unit::CalendarUnit;

/// Resets on calendar boundaries (e.g. the 1st of every month) — not tied to subscription start date.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CalendarResetPeriod {
    pub interval: i32,

    pub unit: CalendarUnit,
}

impl CalendarResetPeriod {
    pub fn new(interval: i32, unit: CalendarUnit) -> Self {
        Self { interval, unit }
    }
}
