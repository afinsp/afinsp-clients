// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PerUnitPlanPricing {
    pub rate: rust_decimal::Decimal,
}

impl PerUnitPlanPricing {
    pub fn new(rate: rust_decimal::Decimal) -> Self {
        Self { rate }
    }
}
