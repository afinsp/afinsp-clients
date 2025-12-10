// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MeteredEntitlementUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_at: Option<String>,
}

impl MeteredEntitlementUsage {
    pub fn new() -> Self {
        Self {
            consumed: None,
            remaining: None,
            reset_at: None,
        }
    }
}
