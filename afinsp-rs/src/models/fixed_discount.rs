// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FixedDiscount {
    pub amount: String,

    pub currency: String,
}

impl FixedDiscount {
    pub fn new(amount: String, currency: String) -> Self {
        Self { amount, currency }
    }
}
