// this file is @generated
use serde::{Deserialize, Serialize};

use super::metric_usage::MetricUsage;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UsageResponse {
    pub period_end: String,

    pub period_start: String,

    pub usage: Vec<MetricUsage>,
}

impl UsageResponse {
    pub fn new(period_end: String, period_start: String, usage: Vec<MetricUsage>) -> Self {
        Self {
            period_end,
            period_start,
            usage,
        }
    }
}
