// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    metered_entitlement_spec::MeteredEntitlementSpec,
    metered_entitlement_usage::MeteredEntitlementUsage,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MeteredEffectiveEntitlementValue {
    pub spec: MeteredEntitlementSpec,

    pub usage: MeteredEntitlementUsage,
}

impl MeteredEffectiveEntitlementValue {
    pub fn new(spec: MeteredEntitlementSpec, usage: MeteredEntitlementUsage) -> Self {
        Self { spec, usage }
    }
}
