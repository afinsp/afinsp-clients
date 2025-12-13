// this file is @generated
use serde::{Deserialize, Serialize};

use super::{pagination_response::PaginationResponse, plan_version_summary::PlanVersionSummary};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlanVersionListResponse {
    pub data: Vec<PlanVersionSummary>,

    pub pagination_meta: PaginationResponse,
}

impl PlanVersionListResponse {
    pub fn new(data: Vec<PlanVersionSummary>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
