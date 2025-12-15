// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    add_on_id::AddOnId, subscription_add_on_id::SubscriptionAddOnId,
    subscription_fee::SubscriptionFee,
    subscription_fee_billing_period_enum::SubscriptionFeeBillingPeriodEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionAddOn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_on_id: Option<AddOnId>,

    pub fee: SubscriptionFee,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SubscriptionAddOnId>,

    pub name: String,

    pub period: SubscriptionFeeBillingPeriodEnum,

    pub quantity: i32,
}

impl SubscriptionAddOn {
    pub fn new(
        fee: SubscriptionFee,
        name: String,
        period: SubscriptionFeeBillingPeriodEnum,
        quantity: i32,
    ) -> Self {
        Self {
            add_on_id: None,
            fee,
            id: None,
            name,
            period,
            quantity,
        }
    }
}
