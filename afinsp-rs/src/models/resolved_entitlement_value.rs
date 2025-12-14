// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    boolean_resolved_entitlement_value::BooleanResolvedEntitlementValue,
    config_resolved_entitlement_value::ConfigResolvedEntitlementValue,
    metered_resolved_entitlement_value::MeteredResolvedEntitlementValue,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ResolvedEntitlementValue {
    #[serde(rename = "BOOLEAN")]
    Boolean(BooleanResolvedEntitlementValue),
    #[serde(rename = "METERED")]
    Metered(MeteredResolvedEntitlementValue),
    #[serde(rename = "CONFIG")]
    Config(ConfigResolvedEntitlementValue),
}

impl Default for ResolvedEntitlementValue {
    fn default() -> Self {
        Self::Boolean(BooleanResolvedEntitlementValue::default())
    }
}
