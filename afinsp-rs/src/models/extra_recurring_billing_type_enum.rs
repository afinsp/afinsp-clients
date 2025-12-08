// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum ExtraRecurringBillingTypeEnum {
    #[default]
    #[serde(rename = "ADVANCE")]
    Advance,

    #[serde(rename = "ARREARS")]
    Arrears,
}

impl fmt::Display for ExtraRecurringBillingTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Advance => "ADVANCE",
            Self::Arrears => "ARREARS",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for ExtraRecurringBillingTypeEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
