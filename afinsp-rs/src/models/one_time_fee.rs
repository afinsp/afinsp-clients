// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OneTimeFee {
    pub quantity: i32,

    pub rate: rust_decimal::Decimal,
}

impl OneTimeFee {
    pub fn new(quantity: i32, rate: rust_decimal::Decimal) -> Self {
        Self { quantity, rate }
    }
}
