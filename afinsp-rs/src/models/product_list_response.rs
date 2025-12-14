// this file is @generated
use serde::{Deserialize, Serialize};

use super::{pagination_response::PaginationResponse, product::Product};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductListResponse {
    pub data: Vec<Product>,

    pub pagination_meta: PaginationResponse,
}

impl ProductListResponse {
    pub fn new(data: Vec<Product>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
