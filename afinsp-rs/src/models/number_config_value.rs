// this file is @generated
use serde::{Deserialize, Serialize};

/// A number config value (decimal, encoded as a string).
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NumberConfigValue {
    pub value: rust_decimal::Decimal,
}

impl NumberConfigValue {
    pub fn new(value: rust_decimal::Decimal) -> Self {
        Self { value }
    }
}
