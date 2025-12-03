// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum BillingMetricAggregateEnum {
    #[default]
    #[serde(rename = "COUNT")]
    Count,

    #[serde(rename = "LATEST")]
    Latest,

    #[serde(rename = "MAX")]
    Max,

    #[serde(rename = "MIN")]
    Min,

    #[serde(rename = "MEAN")]
    Mean,

    #[serde(rename = "SUM")]
    Sum,

    #[serde(rename = "COUNT_DISTINCT")]
    CountDistinct,
}

impl fmt::Display for BillingMetricAggregateEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Count => "COUNT",
            Self::Latest => "LATEST",
            Self::Max => "MAX",
            Self::Min => "MIN",
            Self::Mean => "MEAN",
            Self::Sum => "SUM",
            Self::CountDistinct => "COUNT_DISTINCT",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for BillingMetricAggregateEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
