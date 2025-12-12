// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PatchPlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service_rank: Option<i32>,
}

impl PatchPlanRequest {
    pub fn new() -> Self {
        Self {
            description: None,
            name: None,
            self_service_rank: None,
        }
    }
}
