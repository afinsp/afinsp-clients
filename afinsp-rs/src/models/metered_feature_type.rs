// this file is @generated
use serde::{Deserialize, Serialize};

use super::billable_metric_id::BillableMetricId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MeteredFeatureType {
    pub metric_id: BillableMetricId,
}

impl MeteredFeatureType {
    pub fn new(metric_id: BillableMetricId) -> Self {
        Self { metric_id }
    }
}
