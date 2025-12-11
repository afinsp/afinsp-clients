// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    double_segmentation_matrix::DoubleSegmentationMatrix,
    linked_segmentation_matrix::LinkedSegmentationMatrix, metric_dimension::MetricDimension,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MetricSegmentationMatrix {
    #[serde(rename = "SINGLE")]
    Single(MetricDimension),
    #[serde(rename = "DOUBLE")]
    Double(DoubleSegmentationMatrix),
    #[serde(rename = "LINKED")]
    Linked(LinkedSegmentationMatrix),
}

impl Default for MetricSegmentationMatrix {
    fn default() -> Self {
        Self::Single(MetricDimension::default())
    }
}
