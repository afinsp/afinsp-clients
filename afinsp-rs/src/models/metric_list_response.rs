// this file is @generated
use serde::{Deserialize, Serialize};

use super::{metric_summary::MetricSummary, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetricListResponse {
    pub data: Vec<MetricSummary>,

    pub pagination_meta: PaginationResponse,
}

impl MetricListResponse {
    pub fn new(data: Vec<MetricSummary>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
