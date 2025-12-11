// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId, billing_metric_aggregate_enum::BillingMetricAggregateEnum,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetricSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_key: Option<String>,

    pub aggregation_type: BillingMetricAggregateEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,

    pub code: String,

    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub id: BillableMetricId,

    pub name: String,
}

impl MetricSummary {
    pub fn new(
        aggregation_type: BillingMetricAggregateEnum,
        code: String,
        created_at: String,
        id: BillableMetricId,
        name: String,
    ) -> Self {
        Self {
            aggregation_key: None,
            aggregation_type,
            archived_at: None,
            code,
            created_at,
            description: None,
            id,
            name,
        }
    }
}
