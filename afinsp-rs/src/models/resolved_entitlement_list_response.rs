// this file is @generated
use serde::{Deserialize, Serialize};

use super::resolved_entitlement::ResolvedEntitlement;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ResolvedEntitlementListResponse {
    pub data: Vec<ResolvedEntitlement>,
}

impl ResolvedEntitlementListResponse {
    pub fn new(data: Vec<ResolvedEntitlement>) -> Self {
        Self { data }
    }
}
