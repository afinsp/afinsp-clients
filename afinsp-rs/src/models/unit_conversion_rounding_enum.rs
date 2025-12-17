// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum UnitConversionRoundingEnum {
    #[default]
    #[serde(rename = "UP")]
    Up,

    #[serde(rename = "DOWN")]
    Down,

    #[serde(rename = "NEAREST")]
    Nearest,

    #[serde(rename = "NEAREST_HALF")]
    NearestHalf,

    #[serde(rename = "NEAREST_DECILE")]
    NearestDecile,

    #[serde(rename = "NONE")]
    None,
}

impl fmt::Display for UnitConversionRoundingEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Up => "UP",
            Self::Down => "DOWN",
            Self::Nearest => "NEAREST",
            Self::NearestHalf => "NEAREST_HALF",
            Self::NearestDecile => "NEAREST_DECILE",
            Self::None => "NONE",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for UnitConversionRoundingEnum {
    fn encode(&self) -> String {
        self.to_string()
    }
}
