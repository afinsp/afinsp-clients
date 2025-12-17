// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billable_metric_id::BillableMetricId, usage_model_enum::UsageModelEnum};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UsageFeeStructure {
    pub metric_id: BillableMetricId,

    pub model: UsageModelEnum,
}

impl UsageFeeStructure {
    pub fn new(metric_id: BillableMetricId, model: UsageModelEnum) -> Self {
        Self { metric_id, model }
    }
}
