// this file is @generated
use serde::{Deserialize, Serialize};

use super::effective_entitlement::EffectiveEntitlement;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EffectiveEntitlementListResponse {
    pub data: Vec<EffectiveEntitlement>,
}

impl EffectiveEntitlementListResponse {
    pub fn new(data: Vec<EffectiveEntitlement>) -> Self {
        Self { data }
    }
}
