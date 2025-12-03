// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    batch_job_id::BatchJobId, batch_job_status::BatchJobStatus, batch_job_type::BatchJobType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BatchJobResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    pub created_at: String,

    pub created_by: String,

    pub failed_items: i32,

    pub id: BatchJobId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_name: Option<String>,

    pub job_type: BatchJobType,

    pub processed_items: i32,

    pub status: BatchJobStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i32>,
}

impl BatchJobResponse {
    pub fn new(
        created_at: String,
        created_by: String,
        failed_items: i32,
        id: BatchJobId,
        job_type: BatchJobType,
        processed_items: i32,
        status: BatchJobStatus,
    ) -> Self {
        Self {
            completed_at: None,
            created_at,
            created_by,
            failed_items,
            id,
            input_file_name: None,
            job_type,
            processed_items,
            status,
            total_items: None,
        }
    }
}
