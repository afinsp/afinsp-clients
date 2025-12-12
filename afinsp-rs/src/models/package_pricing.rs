// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PackagePricing {
    pub block_size: i32,

    pub rate: rust_decimal::Decimal,
}

impl PackagePricing {
    pub fn new(block_size: i32, rate: rust_decimal::Decimal) -> Self {
        Self { block_size, rate }
    }
}
