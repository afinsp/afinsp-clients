// this file is @generated
use serde::{Deserialize, Serialize};

use super::usage_pricing_model::UsagePricingModel;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UsagePricing {
    pub model: UsagePricingModel,
}

impl UsagePricing {
    pub fn new(model: UsagePricingModel) -> Self {
        Self { model }
    }
}
