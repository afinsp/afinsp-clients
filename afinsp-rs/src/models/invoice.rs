// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    coupon_line_item::CouponLineItem, currency::Currency, customer_details::CustomerDetails,
    customer_id::CustomerId, invoice_id::InvoiceId, invoice_line_item::InvoiceLineItem,
    invoice_payment_status::InvoicePaymentStatus, invoice_status::InvoiceStatus,
    invoice_type::InvoiceType, subscription_id::SubscriptionId,
    tax_breakdown_item::TaxBreakdownItem, transaction::Transaction,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Invoice {
    pub amount_due: i32,

    pub applied_credits: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_invoice_id: Option<InvoiceId>,

    pub coupons: Vec<CouponLineItem>,

    pub created_at: String,

    pub currency: Currency,

    /// User-defined custom property values, keyed by definition `key`.
    pub custom_properties: serde_json::Value,

    pub customer_details: CustomerDetails,

    pub customer_id: CustomerId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<String>,

    pub id: InvoiceId,

    pub invoice_date: String,

    pub invoice_number: String,

    pub invoice_type: InvoiceType,

    pub line_items: Vec<InvoiceLineItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_uncollectible_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    pub net_terms: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_invoice_id: Option<InvoiceId>,

    pub payment_status: InvoicePaymentStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    pub status: InvoiceStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,

    pub subtotal: i32,

    pub subtotal_recurring: i32,

    pub tax_amount: i32,

    pub tax_breakdown: Vec<TaxBreakdownItem>,

    pub total: i32,

    pub transactions: Vec<Transaction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<String>,
}

impl Invoice {
    pub fn new(
        amount_due: i32,
        applied_credits: i32,
        coupons: Vec<CouponLineItem>,
        created_at: String,
        currency: Currency,
        custom_properties: serde_json::Value,
        customer_details: CustomerDetails,
        customer_id: CustomerId,
        id: InvoiceId,
        invoice_date: String,
        invoice_number: String,
        invoice_type: InvoiceType,
        line_items: Vec<InvoiceLineItem>,
        net_terms: i32,
        payment_status: InvoicePaymentStatus,
        status: InvoiceStatus,
        subtotal: i32,
        subtotal_recurring: i32,
        tax_amount: i32,
        tax_breakdown: Vec<TaxBreakdownItem>,
        total: i32,
        transactions: Vec<Transaction>,
    ) -> Self {
        Self {
            amount_due,
            applied_credits,
            child_invoice_id: None,
            coupons,
            created_at,
            currency,
            custom_properties,
            customer_details,
            customer_id,
            due_date: None,
            finalized_at: None,
            id,
            invoice_date,
            invoice_number,
            invoice_type,
            line_items,
            marked_as_uncollectible_at: None,
            memo: None,
            net_terms,
            paid_at: None,
            parent_invoice_id: None,
            payment_status,
            purchase_order: None,
            reference: None,
            status,
            subscription_id: None,
            subtotal,
            subtotal_recurring,
            tax_amount,
            tax_breakdown,
            total,
            transactions,
            updated_at: None,
            voided_at: None,
        }
    }
}
