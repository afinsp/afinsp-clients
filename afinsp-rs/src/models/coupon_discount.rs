// this file is @generated
use serde::{Deserialize, Serialize};

use super::{fixed_discount::FixedDiscount, percentage_discount::PercentageDiscount};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CouponDiscount {
    #[serde(rename = "PERCENTAGE")]
    Percentage(PercentageDiscount),
    #[serde(rename = "FIXED")]
    Fixed(FixedDiscount),
}

impl Default for CouponDiscount {
    fn default() -> Self {
        Self::Percentage(PercentageDiscount::default())
    }
}
