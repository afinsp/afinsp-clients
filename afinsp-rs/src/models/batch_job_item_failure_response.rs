// this file is @generated
use serde::{Deserialize, Serialize};

use super::batch_job_chunk_id::BatchJobChunkId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BatchJobItemFailureResponse {
    pub chunk_id: BatchJobChunkId,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_identifier: Option<String>,

    pub item_index: i32,

    pub reason: String,
}

impl BatchJobItemFailureResponse {
    pub fn new(chunk_id: BatchJobChunkId, id: String, item_index: i32, reason: String) -> Self {
        Self {
            chunk_id,
            id,
            item_identifier: None,
            item_index,
            reason,
        }
    }
}
