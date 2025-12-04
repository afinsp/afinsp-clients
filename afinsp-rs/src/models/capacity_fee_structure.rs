// this file is @generated
use serde::{Deserialize, Serialize};

use super::billable_metric_id::BillableMetricId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CapacityFeeStructure {
    pub metric_id: BillableMetricId,
}

impl CapacityFeeStructure {
    pub fn new(metric_id: BillableMetricId) -> Self {
        Self { metric_id }
    }
}
