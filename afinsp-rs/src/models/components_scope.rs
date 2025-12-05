// this file is @generated
use serde::{Deserialize, Serialize};

/// Component names — matched against `ReplacePlanRequest::components[].name`.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ComponentsScope {
    pub component_names: Vec<String>,
}

impl ComponentsScope {
    pub fn new(component_names: Vec<String>) -> Self {
        Self { component_names }
    }
}
