// this file is @generated
use serde::{Deserialize, Serialize};

use super::plan_version_id::PlanVersionId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlanVersionSummary {
    pub created_at: String,

    pub currency: String,

    pub id: PlanVersionId,

    pub is_draft: bool,

    pub version: i32,
}

impl PlanVersionSummary {
    pub fn new(
        created_at: String,
        currency: String,
        id: PlanVersionId,
        is_draft: bool,
        version: i32,
    ) -> Self {
        Self {
            created_at,
            currency,
            id,
            is_draft,
            version,
        }
    }
}
