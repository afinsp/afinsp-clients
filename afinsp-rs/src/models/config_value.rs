// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    boolean_config_value::BooleanConfigValue, json_config_value::JsonConfigValue,
    number_config_value::NumberConfigValue, text_config_value::TextConfigValue,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum ConfigValue {
    #[serde(rename = "NUMBER")]
    Number(NumberConfigValue),
    #[serde(rename = "BOOLEAN")]
    Boolean(BooleanConfigValue),
    #[serde(rename = "TEXT")]
    Text(TextConfigValue),
    #[serde(rename = "JSON")]
    Json(JsonConfigValue),
}

impl Default for ConfigValue {
    fn default() -> Self {
        Self::Number(NumberConfigValue::default())
    }
}
