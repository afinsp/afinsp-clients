// this file is @generated
use serde::{Deserialize, Serialize};

use super::term_rate::TermRate;

/// Slot-based fee (e.g., per-seat pricing)
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SlotPlanFee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_count: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i32>,

    pub rates: Vec<TermRate>,

    pub slot_unit_name: String,
}

impl SlotPlanFee {
    pub fn new(rates: Vec<TermRate>, slot_unit_name: String) -> Self {
        Self {
            minimum_count: None,
            quota: None,
            rates,
            slot_unit_name,
        }
    }
}
