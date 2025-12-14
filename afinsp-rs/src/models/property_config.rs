// this file is @generated
use serde::{Deserialize, Serialize};

use super::select_option::SelectOption;

/// Type-specific configuration. Only the fields relevant to `property_type` are interpreted.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PropertyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,

    /// Maximum length for `TEXT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,

    /// Inclusive numeric bounds for `NUMBER`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,

    /// Allowed choices for `SINGLE_SELECT` / `MULTI_SELECT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SelectOption>>,
}

impl PropertyConfig {
    pub fn new() -> Self {
        Self {
            max: None,
            max_length: None,
            min: None,
            options: None,
        }
    }
}
