// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RateFee {
    pub rate: rust_decimal::Decimal,
}

impl RateFee {
    pub fn new(rate: rust_decimal::Decimal) -> Self {
        Self { rate }
    }
}
