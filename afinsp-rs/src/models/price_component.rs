// this file is @generated
use serde::{Deserialize, Serialize};

use super::{fee::Fee, price_component_id::PriceComponentId, product_id::ProductId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PriceComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Fee>,

    pub id: PriceComponentId,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,
}

impl PriceComponent {
    pub fn new(id: PriceComponentId, name: String) -> Self {
        Self {
            fee: None,
            id,
            name,
            product_id: None,
        }
    }
}
