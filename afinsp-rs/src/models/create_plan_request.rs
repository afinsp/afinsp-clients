// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    billing_config::BillingConfig, plan_add_on_input::PlanAddOnInput,
    plan_status_enum::PlanStatusEnum, plan_type_enum::PlanTypeEnum,
    price_component_input::PriceComponentInput, product_family_id::ProductFamilyId,
    trial_config::TrialConfig,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreatePlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<Vec<PlanAddOnInput>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingConfig>,

    pub components: Vec<PriceComponentInput>,

    pub currency: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub name: String,

    pub plan_type: PlanTypeEnum,

    pub product_family_id: ProductFamilyId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service_rank: Option<i32>,

    pub status: PlanStatusEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<TrialConfig>,
}

impl CreatePlanRequest {
    pub fn new(
        components: Vec<PriceComponentInput>,
        currency: String,
        name: String,
        plan_type: PlanTypeEnum,
        product_family_id: ProductFamilyId,
        status: PlanStatusEnum,
    ) -> Self {
        Self {
            add_ons: None,
            billing: None,
            components,
            currency,
            description: None,
            name,
            plan_type,
            product_family_id,
            self_service_rank: None,
            status,
            trial: None,
        }
    }
}
