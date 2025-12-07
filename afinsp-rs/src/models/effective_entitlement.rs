// this file is @generated
use serde::{Deserialize, Serialize};

use super::{effective_entitlement_value::EffectiveEntitlementValue, feature_ref::FeatureRef};

/// Merged entitlement value for a feature for a specific customer, enriched with live usage data.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EffectiveEntitlement {
    pub feature: FeatureRef,

    pub value: EffectiveEntitlementValue,
}

impl EffectiveEntitlement {
    pub fn new(feature: FeatureRef, value: EffectiveEntitlementValue) -> Self {
        Self { feature, value }
    }
}
