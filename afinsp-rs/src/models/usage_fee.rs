// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billable_metric_id::BillableMetricId, usage_pricing_model::UsagePricingModel};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UsageFee {
    pub metric_id: BillableMetricId,

    pub model: UsagePricingModel,
}

impl UsageFee {
    pub fn new(metric_id: BillableMetricId, model: UsagePricingModel) -> Self {
        Self { metric_id, model }
    }
}
