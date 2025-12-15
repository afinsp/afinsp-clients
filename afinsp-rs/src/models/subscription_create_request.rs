// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    create_subscription_add_on::CreateSubscriptionAddOn,
    create_subscription_components::CreateSubscriptionComponents,
    payment_methods_config::PaymentMethodsConfig, plan_id::PlanId,
    subscription_activation_condition_enum::SubscriptionActivationConditionEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionCreateRequest {
    pub activation_condition: SubscriptionActivationConditionEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<CreateSubscriptionAddOn>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance_invoices: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day_anchor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_automatically: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_codes: Option<Vec<String>>,

    /// User-defined custom property values, keyed by definition `key`. Validated against the
    /// tenant's subscription definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,

    pub customer_id_or_alias: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_terms: Option<i32>,

    /// Payment methods configuration. If not specified, inherits from the invoicing entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods_config: Option<PaymentMethodsConfig>,

    pub plan_id: PlanId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_components: Option<CreateSubscriptionComponents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    /// Migration mode: when true with a past start_date, skip creating invoices for past cycles.
    /// The subscription will be set to the current billing period with correct cycle_index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_past_invoices: Option<bool>,

    pub start_date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl SubscriptionCreateRequest {
    pub fn new(
        activation_condition: SubscriptionActivationConditionEnum,
        customer_id_or_alias: String,
        plan_id: PlanId,
        start_date: String,
    ) -> Self {
        Self {
            activation_condition,
            add_ons: None,
            auto_advance_invoices: None,
            billing_day_anchor: None,
            charge_automatically: None,
            coupon_codes: None,
            custom_properties: None,
            customer_id_or_alias,
            end_date: None,
            invoice_memo: None,
            net_terms: None,
            payment_methods_config: None,
            plan_id,
            price_components: None,
            purchase_order: None,
            skip_past_invoices: None,
            start_date,
            trial_days: None,
            version: None,
        }
    }
}
