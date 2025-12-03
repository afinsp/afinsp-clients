// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum BillingPeriodEnum {
    #[default]
    #[serde(rename = "MONTHLY")]
    Monthly,

    #[serde(rename = "QUARTERLY")]
    Quarterly,

    #[serde(rename = "SEMIANNUAL")]
    Semiannual,

    #[serde(rename = "ANNUAL")]
    Annual,
}

impl fmt::Display for BillingPeriodEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Monthly => "MONTHLY",
            Self::Quarterly => "QUARTERLY",
            Self::Semiannual => "SEMIANNUAL",
            Self::Annual => "ANNUAL",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for BillingPeriodEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
