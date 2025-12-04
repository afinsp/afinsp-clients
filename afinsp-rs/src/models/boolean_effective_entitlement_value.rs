// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BooleanEffectiveEntitlementValue {
    pub enabled: bool,
}

impl BooleanEffectiveEntitlementValue {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
