// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum CustomPropertyEntityType {
    #[default]
    #[serde(rename = "CUSTOMER")]
    Customer,

    #[serde(rename = "SUBSCRIPTION")]
    Subscription,

    #[serde(rename = "INVOICE")]
    Invoice,

    #[serde(rename = "CREDIT_NOTE")]
    CreditNote,
}

impl fmt::Display for CustomPropertyEntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Customer => "CUSTOMER",
            Self::Subscription => "SUBSCRIPTION",
            Self::Invoice => "INVOICE",
            Self::CreditNote => "CREDIT_NOTE",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for CustomPropertyEntityType {
    fn encode(&self) -> String {
        self.to_string()
    }
}
