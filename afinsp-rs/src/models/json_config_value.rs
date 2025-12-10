// this file is @generated
use serde::{Deserialize, Serialize};

/// A structured (JSON) config value — the "metadata" case, several fields in one entitlement.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct JsonConfigValue {
    pub value: serde_json::Value,
}

impl JsonConfigValue {
    pub fn new(value: serde_json::Value) -> Self {
        Self { value }
    }
}
