// this file is @generated
use serde::{Deserialize, Serialize};

use super::{customer::Customer, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomerListResponse {
    pub data: Vec<Customer>,

    pub pagination_meta: PaginationResponse,
}

impl CustomerListResponse {
    pub fn new(data: Vec<Customer>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
