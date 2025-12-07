// this file is @generated
use serde::{Deserialize, Serialize};

use super::metric_dimension::MetricDimension;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DoubleSegmentationMatrix {
    pub dimension1: MetricDimension,

    pub dimension2: MetricDimension,
}

impl DoubleSegmentationMatrix {
    pub fn new(dimension1: MetricDimension, dimension2: MetricDimension) -> Self {
        Self {
            dimension1,
            dimension2,
        }
    }
}
