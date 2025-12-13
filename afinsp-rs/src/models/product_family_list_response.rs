// this file is @generated
use serde::{Deserialize, Serialize};

use super::{pagination_response::PaginationResponse, product_family::ProductFamily};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ProductFamilyListResponse {
    pub data: Vec<ProductFamily>,

    pub pagination_meta: PaginationResponse,
}

impl ProductFamilyListResponse {
    pub fn new(data: Vec<ProductFamily>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
