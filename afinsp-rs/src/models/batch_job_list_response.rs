// this file is @generated
use serde::{Deserialize, Serialize};

use super::{batch_job_response::BatchJobResponse, pagination_response::PaginationResponse};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BatchJobListResponse {
    pub data: Vec<BatchJobResponse>,

    pub pagination_meta: PaginationResponse,
}

impl BatchJobListResponse {
    pub fn new(data: Vec<BatchJobResponse>, pagination_meta: PaginationResponse) -> Self {
        Self {
            data,
            pagination_meta,
        }
    }
}
