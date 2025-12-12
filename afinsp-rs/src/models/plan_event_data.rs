// this file is @generated
use serde::{Deserialize, Serialize};

use super::{plan_id::PlanId, plan_status_enum::PlanStatusEnum, plan_type_enum::PlanTypeEnum};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlanEventData {
    pub created_at: String,

    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub name: String,

    pub plan_id: PlanId,

    pub plan_type: PlanTypeEnum,

    pub status: PlanStatusEnum,

    pub version: i32,
}

impl PlanEventData {
    pub fn new(
        created_at: String,
        currency: String,
        name: String,
        plan_id: PlanId,
        plan_type: PlanTypeEnum,
        status: PlanStatusEnum,
        version: i32,
    ) -> Self {
        Self {
            created_at,
            currency,
            description: None,
            name,
            plan_id,
            plan_type,
            status,
            version,
        }
    }
}
