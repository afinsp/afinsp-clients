// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MetricDimension {
    pub key: String,

    pub values: Vec<String>,
}

impl MetricDimension {
    pub fn new(key: String, values: Vec<String>) -> Self {
        Self { key, values }
    }
}
