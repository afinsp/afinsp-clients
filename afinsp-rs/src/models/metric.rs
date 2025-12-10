// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billable_metric_id::BillableMetricId,
    billing_metric_aggregate_enum::BillingMetricAggregateEnum, metric_filter::MetricFilter,
    metric_segmentation_matrix::MetricSegmentationMatrix, product_family_id::ProductFamilyId,
    product_id::ProductId, unit_conversion::UnitConversion,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Metric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_key: Option<String>,

    pub aggregation_type: BillingMetricAggregateEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,

    pub code: String,

    pub created_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MetricFilter>>,

    pub id: BillableMetricId,

    pub name: String,

    pub product_family_id: ProductFamilyId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_matrix: Option<MetricSegmentationMatrix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_conversion: Option<UnitConversion>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_group_key: Option<String>,
}

impl Metric {
    pub fn new(
        aggregation_type: BillingMetricAggregateEnum,
        code: String,
        created_at: String,
        id: BillableMetricId,
        name: String,
        product_family_id: ProductFamilyId,
    ) -> Self {
        Self {
            aggregation_key: None,
            aggregation_type,
            archived_at: None,
            code,
            created_at,
            description: None,
            filters: None,
            id,
            name,
            product_family_id,
            product_id: None,
            segmentation_matrix: None,
            unit_conversion: None,
            usage_group_key: None,
        }
    }
}
