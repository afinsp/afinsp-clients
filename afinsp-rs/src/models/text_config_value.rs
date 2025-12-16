// this file is @generated
use serde::{Deserialize, Serialize};

/// A text config value.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TextConfigValue {
    pub value: String,
}

impl TextConfigValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
