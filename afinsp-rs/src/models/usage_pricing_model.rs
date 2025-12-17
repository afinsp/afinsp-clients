// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    matrix_pricing::MatrixPricing, package_pricing::PackagePricing,
    per_unit_pricing::PerUnitPricing, tiered_pricing::TieredPricing, volume_pricing::VolumePricing,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum UsagePricingModel {
    #[serde(rename = "PER_UNIT")]
    PerUnit(PerUnitPricing),
    #[serde(rename = "TIERED")]
    Tiered(TieredPricing),
    #[serde(rename = "VOLUME")]
    Volume(VolumePricing),
    #[serde(rename = "PACKAGE")]
    Package(PackagePricing),
    #[serde(rename = "MATRIX")]
    Matrix(MatrixPricing),
}

impl Default for UsagePricingModel {
    fn default() -> Self {
        Self::PerUnit(PerUnitPricing::default())
    }
}
