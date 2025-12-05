// this file is @generated
use serde::{Deserialize, Serialize};

use super::config_value_type::ConfigValueType;

/// A static, typed configuration value. No metric — resolved synchronously.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConfigFeatureType {
    /// Allowed values when `value_type = SELECT`. Empty otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    /// The feature's value type, fixed at creation.
    pub value_type: ConfigValueType,
}

impl ConfigFeatureType {
    pub fn new(value_type: ConfigValueType) -> Self {
        Self {
            options: None,
            value_type,
        }
    }
}
