// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SlotPricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slots: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_slots: Option<i32>,

    pub unit_rate: rust_decimal::Decimal,
}

impl SlotPricing {
    pub fn new(unit_rate: rust_decimal::Decimal) -> Self {
        Self {
            max_slots: None,
            min_slots: None,
            unit_rate,
        }
    }
}
