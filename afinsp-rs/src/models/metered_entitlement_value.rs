// this file is @generated
use serde::{Deserialize, Serialize};

use super::reset_period::ResetPeriod;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MeteredEntitlementValue {
    /// Per-entitlement kill switch. `false` means disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Cap on usage. Null means unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<rust_decimal::Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_period: Option<ResetPeriod>,
}

impl MeteredEntitlementValue {
    pub fn new() -> Self {
        Self {
            enabled: None,
            limit: None,
            reset_period: None,
        }
    }
}
