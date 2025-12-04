// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, billing_period_enum::BillingPeriodEnum,
    capacity_threshold::CapacityThreshold,
};

/// Capacity-based fee with included committed usage and overage
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CapacityPlanFee {
    pub cadence: BillingPeriodEnum,

    pub metric_id: BillableMetricId,

    pub thresholds: Vec<CapacityThreshold>,
}

impl CapacityPlanFee {
    pub fn new(
        cadence: BillingPeriodEnum,
        metric_id: BillableMetricId,
        thresholds: Vec<CapacityThreshold>,
    ) -> Self {
        Self {
            cadence,
            metric_id,
            thresholds,
        }
    }
}
