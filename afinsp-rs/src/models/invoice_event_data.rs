// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer_id::CustomerId, invoice_id::InvoiceId, invoice_status::InvoiceStatus};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InvoiceEventData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidated_into_invoice_id: Option<InvoiceId>,

    pub created_at: String,

    pub currency: String,

    /// User-defined custom property values, keyed by definition key.
    pub custom_properties: serde_json::Value,

    pub customer_id: CustomerId,

    pub invoice_id: InvoiceId,

    /// Absent while the invoice is a draft — the number is assigned at finalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_invoice_id: Option<InvoiceId>,

    pub status: InvoiceStatus,

    pub tax_amount: i32,

    pub total: i32,
}

impl InvoiceEventData {
    pub fn new(
        created_at: String,
        currency: String,
        custom_properties: serde_json::Value,
        customer_id: CustomerId,
        invoice_id: InvoiceId,
        status: InvoiceStatus,
        tax_amount: i32,
        total: i32,
    ) -> Self {
        Self {
            consolidated_into_invoice_id: None,
            created_at,
            currency,
            custom_properties,
            customer_id,
            invoice_id,
            invoice_number: None,
            parent_invoice_id: None,
            status,
            tax_amount,
            total,
        }
    }
}
