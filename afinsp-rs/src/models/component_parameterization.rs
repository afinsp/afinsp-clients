// this file is @generated
use serde::{Deserialize, Serialize};

use super::{component_parameters::ComponentParameters, price_component_id::PriceComponentId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ComponentParameterization {
    pub component_id: PriceComponentId,

    pub parameters: ComponentParameters,
}

impl ComponentParameterization {
    pub fn new(component_id: PriceComponentId, parameters: ComponentParameters) -> Self {
        Self {
            component_id,
            parameters,
        }
    }
}
