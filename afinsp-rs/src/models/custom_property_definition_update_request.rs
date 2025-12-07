// this file is @generated
use serde::{Deserialize, Serialize};

use super::property_config::PropertyConfig;

/// Update of a definition. `key`, `entity_type` and `property_type` are immutable and cannot be
/// changed here. Any field left absent is unchanged.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomPropertyDefinitionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<PropertyConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CustomPropertyDefinitionUpdateRequest {
    pub fn new() -> Self {
        Self {
            config: None,
            default_value: None,
            description: None,
            display_order: None,
            name: None,
            required: None,
        }
    }
}
