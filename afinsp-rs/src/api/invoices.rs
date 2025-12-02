// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct InvoicesListInvoicesOptions {
    /// Filter by customer ID or alias
    pub customer_id: Option<String>,

    pub subscription_id: Option<SubscriptionId>,

    pub statuses: Option<Vec<InvoiceStatus>>,

    /// Sort order. Format: `column.direction`. Allowed columns: `invoice_number`, `customer_name`, `amount`, `invoice_date`, `status`, `payment_status`. Direction: `asc` or `desc`. Default: `invoice_date.desc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct Invoices<'a> {
    cfg: &'a Configuration,
}

impl<'a> Invoices<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List invoices with optional filtering by customer, subscription, or status.
    pub async fn list_invoices(
        &self,
        options: Option<InvoicesListInvoicesOptions>,
    ) -> Result<crate::models::InvoiceListResponse> {
        let InvoicesListInvoicesOptions {
            customer_id,
            subscription_id,
            statuses,
            order_by,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/invoices")
            .with_optional_query_param("customer_id", customer_id)
            .with_optional_query_param("subscription_id", subscription_id)
            .with_optional_exploded_query_param("statuses", statuses)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    /// Retrieve a single invoice with its payment transactions.
    pub async fn get_invoice_by_id(&self, invoice_id: String) -> Result<crate::models::Invoice> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/invoices/{invoice_id}")
            .with_path_param("invoice_id", invoice_id)
            .execute(self.cfg)
            .await
    }

    /// Merge custom property values onto an invoice (send a key with `null` to remove it).
    /// Values are validated against the tenant's `INVOICE` property definitions. Allowed at any
    /// status — custom properties are external workflow metadata and stay editable after the invoice
    /// is finalized.
    pub async fn patch_invoice_custom_properties(
        &self,
        invoice_id: String,
        invoice_custom_properties_request: crate::models::InvoiceCustomPropertiesRequest,
    ) -> Result<crate::models::Invoice> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/invoices/{invoice_id}/custom-properties",
        )
        .with_path_param("invoice_id", invoice_id)
        .with_body_param(invoice_custom_properties_request)
        .execute(self.cfg)
        .await
    }

    /// Download the PDF document for an invoice.
    pub async fn download_invoice_pdf(&self, invoice_id: String) -> Result<bytes::Bytes> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/invoices/{invoice_id}/download")
            .with_path_param("invoice_id", invoice_id)
            .execute_binary(self.cfg)
            .await
    }
}
