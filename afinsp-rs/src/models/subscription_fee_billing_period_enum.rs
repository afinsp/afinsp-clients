// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SubscriptionFeeBillingPeriodEnum {
    #[default]
    #[serde(rename = "ONE_TIME")]
    OneTime,

    #[serde(rename = "MONTHLY")]
    Monthly,

    #[serde(rename = "QUARTERLY")]
    Quarterly,

    #[serde(rename = "SEMIANNUAL")]
    Semiannual,

    #[serde(rename = "ANNUAL")]
    Annual,
}

impl fmt::Display for SubscriptionFeeBillingPeriodEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::OneTime => "ONE_TIME",
            Self::Monthly => "MONTHLY",
            Self::Quarterly => "QUARTERLY",
            Self::Semiannual => "SEMIANNUAL",
            Self::Annual => "ANNUAL",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for SubscriptionFeeBillingPeriodEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
