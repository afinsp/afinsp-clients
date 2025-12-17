// this file is @generated
use serde::{Deserialize, Serialize};

use super::unit_conversion_rounding_enum::UnitConversionRoundingEnum;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UnitConversion {
    pub factor: i32,

    pub rounding: UnitConversionRoundingEnum,
}

impl UnitConversion {
    pub fn new(factor: i32, rounding: UnitConversionRoundingEnum) -> Self {
        Self { factor, rounding }
    }
}
