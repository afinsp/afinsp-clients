// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GroupedUsage {
    pub dimensions: std::collections::HashMap<String, String>,

    pub value: rust_decimal::Decimal,
}

impl GroupedUsage {
    pub fn new(
        dimensions: std::collections::HashMap<String, String>,
        value: rust_decimal::Decimal,
    ) -> Self {
        Self { dimensions, value }
    }
}
