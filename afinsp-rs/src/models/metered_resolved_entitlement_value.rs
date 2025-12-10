// this file is @generated
use serde::{Deserialize, Serialize};

use super::{billable_metric_id::BillableMetricId, reset_period::ResetPeriod};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MeteredResolvedEntitlementValue {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<rust_decimal::Decimal>,

    pub metric_id: BillableMetricId,

    pub reset_period: ResetPeriod,
}

impl MeteredResolvedEntitlementValue {
    pub fn new(enabled: bool, metric_id: BillableMetricId, reset_period: ResetPeriod) -> Self {
        Self {
            enabled,
            limit: None,
            metric_id,
            reset_period,
        }
    }
}
