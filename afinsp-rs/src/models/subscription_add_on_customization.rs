// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    subscription_add_on_parameterization::SubscriptionAddOnParameterization,
    subscription_add_on_price_override::SubscriptionAddOnPriceOverride,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SubscriptionAddOnCustomization {
    #[serde(rename = "PRICE_OVERRIDE")]
    PriceOverride(SubscriptionAddOnPriceOverride),
    #[serde(rename = "PARAMETERIZATION")]
    Parameterization(SubscriptionAddOnParameterization),
}

impl Default for SubscriptionAddOnCustomization {
    fn default() -> Self {
        Self::PriceOverride(SubscriptionAddOnPriceOverride::default())
    }
}
