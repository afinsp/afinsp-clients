// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubLineItem {
    pub id: String,

    pub name: String,

    pub quantity: rust_decimal::Decimal,

    pub total: i32,

    pub unit_price: rust_decimal::Decimal,
}

impl SubLineItem {
    pub fn new(
        id: String,
        name: String,
        quantity: rust_decimal::Decimal,
        total: i32,
        unit_price: rust_decimal::Decimal,
    ) -> Self {
        Self {
            id,
            name,
            quantity,
            total,
            unit_price,
        }
    }
}
