// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_period_enum::BillingPeriodEnum, customer_id::CustomerId,
    subscription_id::SubscriptionId, subscription_status_enum::SubscriptionStatusEnum,
    subscription_update_type::SubscriptionUpdateType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscriptionEventData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<String>,

    pub auto_advance_invoices: bool,

    pub billing_day_anchor: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_start_date: Option<String>,

    /// Present on `subscription.cancelled` when a reason was supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<SubscriptionUpdateType>,

    pub charge_automatically: bool,

    pub created_at: String,

    pub currency: String,

    /// User-defined custom property values, keyed by definition key.
    pub custom_properties: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_alias: Option<String>,

    pub customer_id: CustomerId,

    pub customer_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_threshold: Option<String>,

    pub mrr_cents: i32,

    pub net_terms: i32,

    pub period: BillingPeriodEnum,

    pub plan_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    pub start_date: String,

    pub status: SubscriptionStatusEnum,

    pub subscription_id: SubscriptionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_duration: Option<i32>,

    pub version: i32,
}

impl SubscriptionEventData {
    pub fn new(
        auto_advance_invoices: bool,
        billing_day_anchor: i32,
        charge_automatically: bool,
        created_at: String,
        currency: String,
        custom_properties: serde_json::Value,
        customer_id: CustomerId,
        customer_name: String,
        mrr_cents: i32,
        net_terms: i32,
        period: BillingPeriodEnum,
        plan_name: String,
        start_date: String,
        status: SubscriptionStatusEnum,
        subscription_id: SubscriptionId,
        version: i32,
    ) -> Self {
        Self {
            activated_at: None,
            auto_advance_invoices,
            billing_day_anchor,
            billing_start_date: None,
            cancellation_reason: None,
            change_type: None,
            charge_automatically,
            created_at,
            currency,
            custom_properties,
            customer_alias: None,
            customer_id,
            customer_name,
            end_date: None,
            invoice_memo: None,
            invoice_threshold: None,
            mrr_cents,
            net_terms,
            period,
            plan_name,
            purchase_order: None,
            start_date,
            status,
            subscription_id,
            trial_duration: None,
            version,
        }
    }
}
