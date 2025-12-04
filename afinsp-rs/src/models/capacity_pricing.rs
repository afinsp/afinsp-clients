// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CapacityPricing {
    pub included: i32,

    pub overage_rate: rust_decimal::Decimal,

    pub rate: rust_decimal::Decimal,
}

impl CapacityPricing {
    pub fn new(
        included: i32,
        overage_rate: rust_decimal::Decimal,
        rate: rust_decimal::Decimal,
    ) -> Self {
        Self {
            included,
            overage_rate,
            rate,
        }
    }
}
