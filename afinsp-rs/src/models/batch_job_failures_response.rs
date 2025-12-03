// this file is @generated
use serde::{Deserialize, Serialize};

use super::batch_job_item_failure_response::BatchJobItemFailureResponse;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BatchJobFailuresResponse {
    pub data: Vec<BatchJobItemFailureResponse>,

    pub total_count: i32,
}

impl BatchJobFailuresResponse {
    pub fn new(data: Vec<BatchJobItemFailureResponse>, total_count: i32) -> Self {
        Self { data, total_count }
    }
}
