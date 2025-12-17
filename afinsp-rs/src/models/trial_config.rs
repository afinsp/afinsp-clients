// this file is @generated
use serde::{Deserialize, Serialize};

use super::plan_id::PlanId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TrialConfig {
    pub duration_days: i32,

    pub is_free: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trialing_plan_id: Option<PlanId>,
}

impl TrialConfig {
    pub fn new(duration_days: i32, is_free: bool) -> Self {
        Self {
            duration_days,
            is_free,
            trialing_plan_id: None,
        }
    }
}
