// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    component_override::ComponentOverride, component_parameterization::ComponentParameterization,
    extra_component::ExtraComponent, price_component_id::PriceComponentId,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CreateSubscriptionComponents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_components: Option<Vec<ExtraComponent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_components: Option<Vec<ComponentOverride>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameterized_components: Option<Vec<ComponentParameterization>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_components: Option<Vec<PriceComponentId>>,
}

impl CreateSubscriptionComponents {
    pub fn new() -> Self {
        Self {
            extra_components: None,
            overridden_components: None,
            parameterized_components: None,
            remove_components: None,
        }
    }
}
