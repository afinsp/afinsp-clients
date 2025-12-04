// this file is @generated
use serde::{Deserialize, Serialize};

/// A boolean config value.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BooleanConfigValue {
    pub value: bool,
}

impl BooleanConfigValue {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}
