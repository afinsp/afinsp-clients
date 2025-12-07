// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    custom_property_definition_id::CustomPropertyDefinitionId,
    custom_property_entity_type::CustomPropertyEntityType,
    custom_property_type::CustomPropertyType, property_config::PropertyConfig,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomPropertyDefinition {
    pub archived: bool,

    pub config: PropertyConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub display_order: i32,

    pub entity_type: CustomPropertyEntityType,

    pub id: CustomPropertyDefinitionId,

    pub key: String,

    pub name: String,

    pub property_type: CustomPropertyType,

    pub required: bool,
}

impl CustomPropertyDefinition {
    pub fn new(
        archived: bool,
        config: PropertyConfig,
        display_order: i32,
        entity_type: CustomPropertyEntityType,
        id: CustomPropertyDefinitionId,
        key: String,
        name: String,
        property_type: CustomPropertyType,
        required: bool,
    ) -> Self {
        Self {
            archived,
            config,
            default_value: None,
            description: None,
            display_order,
            entity_type,
            id,
            key,
            name,
            property_type,
            required,
        }
    }
}
