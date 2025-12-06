// this file is @generated
use serde::{Deserialize, Serialize};

use super::{credit_note::CreditNote, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreditNoteListResponse {
    pub data: Vec<CreditNote>,

    pub pagination_meta: PaginationResponse,
}

impl CreditNoteListResponse {
    pub fn new(data: Vec<CreditNote>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
