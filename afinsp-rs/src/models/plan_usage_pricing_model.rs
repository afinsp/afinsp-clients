// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    matrix_plan_pricing::MatrixPlanPricing, package_plan_pricing::PackagePlanPricing,
    per_unit_plan_pricing::PerUnitPlanPricing, tiered_plan_pricing::TieredPlanPricing,
    volume_plan_pricing::VolumePlanPricing,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum PlanUsagePricingModel {
    #[serde(rename = "PER_UNIT")]
    PerUnit(PerUnitPlanPricing),
    #[serde(rename = "TIERED")]
    Tiered(TieredPlanPricing),
    #[serde(rename = "VOLUME")]
    Volume(VolumePlanPricing),
    #[serde(rename = "PACKAGE")]
    Package(PackagePlanPricing),
    #[serde(rename = "MATRIX")]
    Matrix(MatrixPlanPricing),
}

impl Default for PlanUsagePricingModel {
    fn default() -> Self {
        Self::PerUnit(PerUnitPlanPricing::default())
    }
}
