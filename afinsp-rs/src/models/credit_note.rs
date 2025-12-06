// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    credit_note_id::CreditNoteId, credit_note_status::CreditNoteStatus, credit_type::CreditType,
    currency::Currency, customer_id::CustomerId, invoice_id::InvoiceId,
    invoice_line_item::InvoiceLineItem, plan_version_id::PlanVersionId,
    subscription_id::SubscriptionId, tax_breakdown_item::TaxBreakdownItem,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreditNote {
    pub created_at: String,

    pub credit_note_number: String,

    pub credit_type: CreditType,

    pub credited_amount_cents: i32,

    pub currency: Currency,

    /// User-defined custom property values, keyed by definition `key`.
    pub custom_properties: serde_json::Value,

    pub customer_id: CustomerId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<String>,

    pub id: CreditNoteId,

    pub invoice_id: InvoiceId,

    pub invoice_number: String,

    pub line_items: Vec<InvoiceLineItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_version_id: Option<PlanVersionId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    pub refunded_amount_cents: i32,

    pub status: CreditNoteStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,

    pub subtotal: i32,

    pub tax_amount: i32,

    pub tax_breakdown: Vec<TaxBreakdownItem>,

    pub total: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<String>,
}

impl CreditNote {
    pub fn new(
        created_at: String,
        credit_note_number: String,
        credit_type: CreditType,
        credited_amount_cents: i32,
        currency: Currency,
        custom_properties: serde_json::Value,
        customer_id: CustomerId,
        id: CreditNoteId,
        invoice_id: InvoiceId,
        invoice_number: String,
        line_items: Vec<InvoiceLineItem>,
        refunded_amount_cents: i32,
        status: CreditNoteStatus,
        subtotal: i32,
        tax_amount: i32,
        tax_breakdown: Vec<TaxBreakdownItem>,
        total: i32,
    ) -> Self {
        Self {
            created_at,
            credit_note_number,
            credit_type,
            credited_amount_cents,
            currency,
            custom_properties,
            customer_id,
            finalized_at: None,
            id,
            invoice_id,
            invoice_number,
            line_items,
            memo: None,
            plan_version_id: None,
            reason: None,
            refunded_amount_cents,
            status,
            subscription_id: None,
            subtotal,
            tax_amount,
            tax_breakdown,
            total,
            updated_at: None,
            voided_at: None,
        }
    }
}
