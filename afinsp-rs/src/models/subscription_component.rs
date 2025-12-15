// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    price_component_id::PriceComponentId, product_id::ProductId, subscription_fee::SubscriptionFee,
    subscription_fee_billing_period_enum::SubscriptionFeeBillingPeriodEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionComponent {
    pub fee: SubscriptionFee,

    pub name: String,

    pub period: SubscriptionFeeBillingPeriodEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_component_id: Option<PriceComponentId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,
}

impl SubscriptionComponent {
    pub fn new(
        fee: SubscriptionFee,
        name: String,
        period: SubscriptionFeeBillingPeriodEnum,
    ) -> Self {
        Self {
            fee,
            name,
            period,
            price_component_id: None,
            product_id: None,
        }
    }
}
