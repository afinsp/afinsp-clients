// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    batch_job_id::BatchJobId, batch_job_status::BatchJobStatus, batch_job_type::BatchJobType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BatchJobDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    pub created_at: String,

    pub created_by: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_csv_url: Option<String>,

    pub failed_items: i32,

    pub failure_count: i32,

    pub has_error_csv: bool,

    pub has_output: bool,

    pub id: BatchJobId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_url: Option<String>,

    pub job_type: BatchJobType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<String>,

    pub processed_items: i32,

    pub status: BatchJobStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i32>,
}

impl BatchJobDetailResponse {
    pub fn new(
        created_at: String,
        created_by: String,
        failed_items: i32,
        failure_count: i32,
        has_error_csv: bool,
        has_output: bool,
        id: BatchJobId,
        job_type: BatchJobType,
        processed_items: i32,
        status: BatchJobStatus,
    ) -> Self {
        Self {
            completed_at: None,
            created_at,
            created_by,
            error_csv_url: None,
            failed_items,
            failure_count,
            has_error_csv,
            has_output,
            id,
            input_file_name: None,
            input_file_url: None,
            job_type,
            output_url: None,
            processed_items,
            status,
            total_items: None,
        }
    }
}
