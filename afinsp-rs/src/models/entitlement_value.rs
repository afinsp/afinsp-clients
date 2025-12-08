// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    boolean_entitlement_value::BooleanEntitlementValue,
    config_entitlement_value::ConfigEntitlementValue,
    metered_entitlement_value::MeteredEntitlementValue,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EntitlementValue {
    #[serde(rename = "BOOLEAN")]
    Boolean(BooleanEntitlementValue),
    #[serde(rename = "METERED")]
    Metered(MeteredEntitlementValue),
    #[serde(rename = "CONFIG")]
    Config(ConfigEntitlementValue),
}

impl Default for EntitlementValue {
    fn default() -> Self {
        Self::Boolean(BooleanEntitlementValue::default())
    }
}
