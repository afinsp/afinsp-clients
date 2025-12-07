// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    custom_property_entity_type::CustomPropertyEntityType,
    custom_property_type::CustomPropertyType, property_config::PropertyConfig,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomPropertyDefinitionCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<PropertyConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,

    pub entity_type: CustomPropertyEntityType,

    /// Immutable machine name; letters, digits and underscores only. Unique per entity type.
    pub key: String,

    pub name: String,

    pub property_type: CustomPropertyType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CustomPropertyDefinitionCreateRequest {
    pub fn new(
        entity_type: CustomPropertyEntityType,
        key: String,
        name: String,
        property_type: CustomPropertyType,
    ) -> Self {
        Self {
            config: None,
            default_value: None,
            description: None,
            display_order: None,
            entity_type,
            key,
            name,
            property_type,
            required: None,
        }
    }
}
