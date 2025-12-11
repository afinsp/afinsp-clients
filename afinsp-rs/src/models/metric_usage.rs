// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billable_metric_id::BillableMetricId, grouped_usage::GroupedUsage};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetricUsage {
    pub grouped_usage: Vec<GroupedUsage>,

    pub metric_code: String,

    pub metric_id: BillableMetricId,

    pub metric_name: String,

    pub total_value: rust_decimal::Decimal,
}

impl MetricUsage {
    pub fn new(
        grouped_usage: Vec<GroupedUsage>,
        metric_code: String,
        metric_id: BillableMetricId,
        metric_name: String,
        total_value: rust_decimal::Decimal,
    ) -> Self {
        Self {
            grouped_usage,
            metric_code,
            metric_id,
            metric_name,
            total_value,
        }
    }
}
