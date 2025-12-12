// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaginationResponse {
    pub page: i32,

    pub per_page: i32,

    pub total_items: i32,

    pub total_pages: i32,
}

impl PaginationResponse {
    pub fn new(page: i32, per_page: i32, total_items: i32, total_pages: i32) -> Self {
        Self {
            page,
            per_page,
            total_items,
            total_pages,
        }
    }
}
