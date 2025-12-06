// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    coupon_id::CouponId, create_subscription_add_on::CreateSubscriptionAddOn,
    create_subscription_components::CreateSubscriptionComponents,
    payment_methods_config::PaymentMethodsConfig, plan_version_id::PlanVersionId,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateCheckoutSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<CreateSubscriptionAddOn>>,

    /// If false, invoices will stay in Draft until manually reviewed and finalized. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance_invoices: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day_anchor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_start_date: Option<String>,

    /// Automatically try to charge the customer's configured payment method on finalize. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_automatically: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<CreateSubscriptionComponents>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_ids: Option<Vec<CouponId>>,

    /// Customer ID or alias
    pub customer_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    /// Session expiry time in hours. Default is 1 hour for self-serve checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_hours: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_threshold: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_terms: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods_config: Option<PaymentMethodsConfig>,

    pub plan_version_id: PlanVersionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_duration_days: Option<i32>,
}

impl CreateCheckoutSessionRequest {
    pub fn new(customer_id: String, plan_version_id: PlanVersionId) -> Self {
        Self {
            add_ons: None,
            auto_advance_invoices: None,
            billing_day_anchor: None,
            billing_start_date: None,
            charge_automatically: None,
            components: None,
            coupon_code: None,
            coupon_ids: None,
            customer_id,
            end_date: None,
            expires_in_hours: None,
            invoice_memo: None,
            invoice_threshold: None,
            metadata: None,
            net_terms: None,
            payment_methods_config: None,
            plan_version_id,
            purchase_order: None,
            trial_duration_days: None,
        }
    }
}
