// this file is @generated
use serde::{Deserialize, Serialize};

use super::config_value::ConfigValue;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConfigEntitlementValue {
    pub value: ConfigValue,
}

impl ConfigEntitlementValue {
    pub fn new(value: ConfigValue) -> Self {
        Self { value }
    }
}
