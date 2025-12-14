// this file is @generated
use serde::{Deserialize, Serialize};

use super::{feature_ref::FeatureRef, resolved_entitlement_value::ResolvedEntitlementValue};

/// Merged entitlement value for a feature across the priority hierarchy, without usage data.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResolvedEntitlement {
    pub feature: FeatureRef,

    pub value: ResolvedEntitlementValue,
}

impl ResolvedEntitlement {
    pub fn new(feature: FeatureRef, value: ResolvedEntitlementValue) -> Self {
        Self { feature, value }
    }
}
