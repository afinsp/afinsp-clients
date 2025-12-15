// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    applied_coupon_detailed::AppliedCouponDetailed, billing_period_enum::BillingPeriodEnum,
    currency::Currency, customer_id::CustomerId, entitlement::Entitlement,
    minimum_commitment::MinimumCommitment, payment_methods_config::PaymentMethodsConfig,
    plan_id::PlanId, plan_version_id::PlanVersionId, subscription_add_on::SubscriptionAddOn,
    subscription_component::SubscriptionComponent, subscription_id::SubscriptionId,
    subscription_status_enum::SubscriptionStatusEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionDetails {
    /// When the subscription was activated (first payment or activation condition met)
    ///
    /// RFC3339 date string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<String>,

    pub add_ons: Vec<SubscriptionAddOn>,

    pub applied_coupons: Vec<AppliedCouponDetailed>,

    pub auto_advance_invoices: bool,

    pub billing_day_anchor: i32,

    /// When billing started (after any trial period)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_start_date: Option<String>,

    pub charge_automatically: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,

    pub components: Vec<SubscriptionComponent>,

    /// When the subscription was created
    ///
    /// RFC3339 date string.
    pub created_at: String,

    pub currency: Currency,

    /// Current billing period end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,

    /// Current billing period start date
    pub current_period_start: String,

    /// User-defined custom property values, keyed by definition `key`.
    pub custom_properties: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,

    pub customer_id: CustomerId,

    pub customer_name: String,

    /// When the subscription ends (if set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlements: Option<Vec<Entitlement>>,

    pub id: SubscriptionId,

    /// Default memo for invoices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_commitment: Option<MinimumCommitment>,

    /// Monthly recurring revenue in cents
    pub mrr_cents: i32,

    /// Payment terms in days (0 = due on issue)
    pub net_terms: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods_config: Option<PaymentMethodsConfig>,

    /// Billing period (monthly, annual, etc.)
    pub period: BillingPeriodEnum,

    pub plan_id: PlanId,

    pub plan_name: String,

    pub plan_version: i32,

    pub plan_version_id: PlanVersionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    /// When the subscription contract starts (benefits apply from this date)
    pub start_date: String,

    pub status: SubscriptionStatusEnum,

    /// Trial duration in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_duration: Option<i32>,
}

impl SubscriptionDetails {
    pub fn new(
        add_ons: Vec<SubscriptionAddOn>,
        applied_coupons: Vec<AppliedCouponDetailed>,
        auto_advance_invoices: bool,
        billing_day_anchor: i32,
        charge_automatically: bool,
        components: Vec<SubscriptionComponent>,
        created_at: String,
        currency: Currency,
        current_period_start: String,
        custom_properties: serde_json::Value,
        customer_id: CustomerId,
        customer_name: String,
        id: SubscriptionId,
        mrr_cents: i32,
        net_terms: i32,
        period: BillingPeriodEnum,
        plan_id: PlanId,
        plan_name: String,
        plan_version: i32,
        plan_version_id: PlanVersionId,
        start_date: String,
        status: SubscriptionStatusEnum,
    ) -> Self {
        Self {
            activated_at: None,
            add_ons,
            applied_coupons,
            auto_advance_invoices,
            billing_day_anchor,
            billing_start_date: None,
            charge_automatically,
            checkout_url: None,
            components,
            created_at,
            currency,
            current_period_end: None,
            current_period_start,
            custom_properties,
            customer_alias: None,
            customer_id,
            customer_name,
            end_date: None,
            entitlements: None,
            id,
            invoice_memo: None,
            minimum_commitment: None,
            mrr_cents,
            net_terms,
            payment_methods_config: None,
            period,
            plan_id,
            plan_name,
            plan_version,
            plan_version_id,
            purchase_order: None,
            start_date,
            status,
            trial_duration: None,
        }
    }
}
