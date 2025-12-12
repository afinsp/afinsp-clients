// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PercentageDiscount {
    pub percentage: String,
}

impl PercentageDiscount {
    pub fn new(percentage: String) -> Self {
        Self { percentage }
    }
}
