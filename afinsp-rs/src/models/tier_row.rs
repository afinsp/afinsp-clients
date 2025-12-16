// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TierRow {
    pub first_unit: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_cap: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_fee: Option<rust_decimal::Decimal>,

    pub rate: rust_decimal::Decimal,
}

impl TierRow {
    pub fn new(first_unit: i32, rate: rust_decimal::Decimal) -> Self {
        Self {
            first_unit,
            flat_cap: None,
            flat_fee: None,
            rate,
        }
    }
}
