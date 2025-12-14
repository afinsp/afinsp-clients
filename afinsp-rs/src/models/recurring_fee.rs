// this file is @generated
use serde::{Deserialize, Serialize};

use super::billing_type_enum::BillingTypeEnum;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecurringFee {
    pub billing_type: BillingTypeEnum,

    pub quantity: i32,

    pub rate: rust_decimal::Decimal,
}

impl RecurringFee {
    pub fn new(billing_type: BillingTypeEnum, quantity: i32, rate: rust_decimal::Decimal) -> Self {
        Self {
            billing_type,
            quantity,
            rate,
        }
    }
}
