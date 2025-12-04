// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BooleanEntitlementValue {
    pub enabled: bool,
}

impl BooleanEntitlementValue {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
