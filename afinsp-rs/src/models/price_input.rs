// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billing_period_enum::BillingPeriodEnum, pricing::Pricing};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PriceInput {
    pub cadence: BillingPeriodEnum,

    pub currency: String,

    pub pricing: Pricing,
}

impl PriceInput {
    pub fn new(cadence: BillingPeriodEnum, currency: String, pricing: Pricing) -> Self {
        Self {
            cadence,
            currency,
            pricing,
        }
    }
}
