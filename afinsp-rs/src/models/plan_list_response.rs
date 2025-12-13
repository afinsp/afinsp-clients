// this file is @generated
use serde::{Deserialize, Serialize};

use super::{pagination_response::PaginationResponse, plan::Plan};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlanListResponse {
    pub data: Vec<Plan>,

    pub pagination_meta: PaginationResponse,
}

impl PlanListResponse {
    pub fn new(data: Vec<Plan>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
