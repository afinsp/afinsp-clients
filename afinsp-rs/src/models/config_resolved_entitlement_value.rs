// this file is @generated
use serde::{Deserialize, Serialize};

use super::config_value::ConfigValue;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConfigResolvedEntitlementValue {
    pub value: ConfigValue,
}

impl ConfigResolvedEntitlementValue {
    pub fn new(value: ConfigValue) -> Self {
        Self { value }
    }
}
