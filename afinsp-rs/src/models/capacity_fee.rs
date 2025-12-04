// this file is @generated
use serde::{Deserialize, Serialize};

use super::billable_metric_id::BillableMetricId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CapacityFee {
    pub included: i32,

    pub metric_id: BillableMetricId,

    pub overage_rate: rust_decimal::Decimal,

    pub rate: rust_decimal::Decimal,
}

impl CapacityFee {
    pub fn new(
        included: i32,
        metric_id: BillableMetricId,
        overage_rate: rust_decimal::Decimal,
        rate: rust_decimal::Decimal,
    ) -> Self {
        Self {
            included,
            metric_id,
            overage_rate,
            rate,
        }
    }
}
