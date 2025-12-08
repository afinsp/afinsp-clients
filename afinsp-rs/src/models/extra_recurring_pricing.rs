// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ExtraRecurringPricing {
    pub quantity: i32,

    pub unit_price: rust_decimal::Decimal,
}

impl ExtraRecurringPricing {
    pub fn new(quantity: i32, unit_price: rust_decimal::Decimal) -> Self {
        Self {
            quantity,
            unit_price,
        }
    }
}
