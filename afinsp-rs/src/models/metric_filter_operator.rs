// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

/// Operator of a pre-aggregation [`MetricFilter`]. `EQUAL`/`NOT_EQUAL` are the single-value
/// forms of `IN`/`NOT_IN`. Negation (`NOT_EQUAL`/`NOT_IN`) is presence-required: an event
/// missing the property is excluded.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum MetricFilterOperator {
    #[default]
    #[serde(rename = "EQUAL")]
    Equal,

    #[serde(rename = "NOT_EQUAL")]
    NotEqual,

    #[serde(rename = "IN")]
    In,

    #[serde(rename = "NOT_IN")]
    NotIn,
}

impl fmt::Display for MetricFilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Equal => "EQUAL",
            Self::NotEqual => "NOT_EQUAL",
            Self::In => "IN",
            Self::NotIn => "NOT_IN",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for MetricFilterOperator {
    fn encode(&self) -> String {
        self.to_string()
    }
}
