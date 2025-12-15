// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum SlotDowngradePolicyEnum {
    #[default]
    #[serde(rename = "REMOVE_AT_END_OF_PERIOD")]
    RemoveAtEndOfPeriod,
}

impl fmt::Display for SlotDowngradePolicyEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::RemoveAtEndOfPeriod => "REMOVE_AT_END_OF_PERIOD",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for SlotDowngradePolicyEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
