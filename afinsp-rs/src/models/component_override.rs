// this file is @generated
use serde::{Deserialize, Serialize};

use super::{price_component_id::PriceComponentId, price_entry::PriceEntry};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ComponentOverride {
    pub component_id: PriceComponentId,

    pub name: String,

    pub price_entry: PriceEntry,
}

impl ComponentOverride {
    pub fn new(component_id: PriceComponentId, name: String, price_entry: PriceEntry) -> Self {
        Self {
            component_id,
            name,
            price_entry,
        }
    }
}
