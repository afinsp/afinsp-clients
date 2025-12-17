// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, billing_period_enum::BillingPeriodEnum,
    plan_usage_pricing_model::PlanUsagePricingModel,
};

/// Usage-based fee
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UsagePlanFee {
    pub cadence: BillingPeriodEnum,

    pub metric_id: BillableMetricId,

    pub pricing: PlanUsagePricingModel,
}

impl UsagePlanFee {
    pub fn new(
        cadence: BillingPeriodEnum,
        metric_id: BillableMetricId,
        pricing: PlanUsagePricingModel,
    ) -> Self {
        Self {
            cadence,
            metric_id,
            pricing,
        }
    }
}
