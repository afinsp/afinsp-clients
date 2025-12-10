// this file is @generated
use serde::{Deserialize, Serialize};

use super::metric_filter_operator::MetricFilterOperator;

/// A pre-aggregation filter: only events whose `property` matches feed the metric's
/// aggregation. Distinct from a segmentation dimension (which splits pricing). Multiple
/// filters are ANDed.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetricFilter {
    pub op: MetricFilterOperator,

    pub property: String,

    pub values: Vec<String>,
}

impl MetricFilter {
    pub fn new(op: MetricFilterOperator, property: String, values: Vec<String>) -> Self {
        Self {
            op,
            property,
            values,
        }
    }
}
