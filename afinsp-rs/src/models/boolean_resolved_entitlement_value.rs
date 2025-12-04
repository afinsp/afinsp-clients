// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BooleanResolvedEntitlementValue {
    pub enabled: bool,
}

impl BooleanResolvedEntitlementValue {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
