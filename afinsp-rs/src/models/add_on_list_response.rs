// this file is @generated
use serde::{Deserialize, Serialize};

use super::{add_on::AddOn, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AddOnListResponse {
    pub data: Vec<AddOn>,

    pub pagination_meta: PaginationResponse,
}

impl AddOnListResponse {
    pub fn new(data: Vec<AddOn>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
