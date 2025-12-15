// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SlotUpgradePolicyEnum {
    #[default]
    #[serde(rename = "PRORATED")]
    Prorated,
}

impl fmt::Display for SlotUpgradePolicyEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Prorated => "PRORATED",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for SlotUpgradePolicyEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
