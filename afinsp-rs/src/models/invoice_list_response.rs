// this file is @generated
use serde::{Deserialize, Serialize};

use super::{invoice::Invoice, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InvoiceListResponse {
    pub data: Vec<Invoice>,

    pub pagination_meta: PaginationResponse,
}

impl InvoiceListResponse {
    pub fn new(data: Vec<Invoice>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
