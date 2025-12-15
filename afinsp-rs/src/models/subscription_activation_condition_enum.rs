// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SubscriptionActivationConditionEnum {
    #[default]
    #[serde(rename = "ON_START")]
    OnStart,

    #[serde(rename = "ON_CHECKOUT")]
    OnCheckout,

    #[serde(rename = "MANUAL")]
    Manual,
}

impl fmt::Display for SubscriptionActivationConditionEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::OnStart => "ON_START",
            Self::OnCheckout => "ON_CHECKOUT",
            Self::Manual => "MANUAL",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for SubscriptionActivationConditionEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
