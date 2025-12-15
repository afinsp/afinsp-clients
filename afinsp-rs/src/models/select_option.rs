// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SelectOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    pub value: String,
}

impl SelectOption {
    pub fn new(value: String) -> Self {
        Self { label: None, value }
    }
}
