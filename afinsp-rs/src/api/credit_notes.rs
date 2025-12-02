// this file is @generated
#[allow(unused_imports)]
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct CreditNotesListCreditNotesOptions {
    /// Filter by customer ID
    pub customer_id: Option<CustomerId>,

    /// Filter by invoice ID
    pub invoice_id: Option<InvoiceId>,

    pub status: Option<CreditNoteStatus>,

    /// Free-text search over credit note number.
    pub search: Option<String>,

    /// Sort order. Format: `column.direction`. Allowed columns: `created_at`, `credit_note_number`, `total`, `status`. Direction: `asc` or `desc`. Default: `created_at.desc`.
    pub order_by: Option<String>,

    /// Page number (0-indexed)
    pub page: Option<i32>,

    /// Number of items per page
    pub per_page: Option<i32>,
}

pub struct CreditNotes<'a> {
    cfg: &'a Configuration,
}

impl<'a> CreditNotes<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List a tenant's credit notes, optionally filtered by customer, invoice or status.
    pub async fn list_credit_notes(
        &self,
        options: Option<CreditNotesListCreditNotesOptions>,
    ) -> Result<crate::models::CreditNoteListResponse> {
        let CreditNotesListCreditNotesOptions {
            customer_id,
            invoice_id,
            status,
            search,
            order_by,
            page,
            per_page,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/credit-notes")
            .with_optional_query_param("customer_id", customer_id)
            .with_optional_query_param("invoice_id", invoice_id)
            .with_optional_query_param("status", status)
            .with_optional_query_param("search", search)
            .with_optional_query_param("order_by", order_by)
            .with_optional_query_param("page", page)
            .with_optional_query_param("per_page", per_page)
            .execute(self.cfg)
            .await
    }

    /// Retrieve a single credit note by ID.
    pub async fn get_credit_note_by_id(
        &self,
        credit_note_id: String,
    ) -> Result<crate::models::CreditNote> {
        crate::request::Request::new(http1::Method::GET, "/api/v1/credit-notes/{credit_note_id}")
            .with_path_param("credit_note_id", credit_note_id)
            .execute(self.cfg)
            .await
    }

    /// Merge custom property values onto a credit note (send a key with `null` to remove it).
    /// Values are validated against the tenant's `CREDIT_NOTE` property definitions. Allowed at any
    /// status — custom properties are external workflow metadata and stay editable after the credit
    /// note is finalized.
    pub async fn patch_credit_note_custom_properties(
        &self,
        credit_note_id: String,
        credit_note_custom_properties_request: crate::models::CreditNoteCustomPropertiesRequest,
    ) -> Result<crate::models::CreditNote> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/api/v1/credit-notes/{credit_note_id}/custom-properties",
        )
        .with_path_param("credit_note_id", credit_note_id)
        .with_body_param(credit_note_custom_properties_request)
        .execute(self.cfg)
        .await
    }

    pub async fn download_credit_note_pdf(&self, credit_note_id: String) -> Result<bytes::Bytes> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/credit-notes/{credit_note_id}/download",
        )
        .with_path_param("credit_note_id", credit_note_id)
        .execute_binary(self.cfg)
        .await
    }
}
