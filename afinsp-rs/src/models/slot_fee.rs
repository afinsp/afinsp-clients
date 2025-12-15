// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SlotFee {
    pub initial_slots: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slots: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_slots: Option<i32>,

    pub unit: String,

    pub unit_rate: rust_decimal::Decimal,
}

impl SlotFee {
    pub fn new(initial_slots: i32, unit: String, unit_rate: rust_decimal::Decimal) -> Self {
        Self {
            initial_slots,
            max_slots: None,
            min_slots: None,
            unit,
            unit_rate,
        }
    }
}
