// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billing_period_enum::BillingPeriodEnum, billing_type::BillingType};

/// Extra recurring fee
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ExtraRecurringPlanFee {
    pub billing_type: BillingType,

    pub cadence: BillingPeriodEnum,

    pub quantity: i32,

    pub unit_price: rust_decimal::Decimal,
}

impl ExtraRecurringPlanFee {
    pub fn new(
        billing_type: BillingType,
        cadence: BillingPeriodEnum,
        quantity: i32,
        unit_price: rust_decimal::Decimal,
    ) -> Self {
        Self {
            billing_type,
            cadence,
            quantity,
            unit_price,
        }
    }
}
