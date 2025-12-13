// this file is @generated
use serde::{Deserialize, Serialize};

use super::{fee::Fee, product_id::ProductId};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PriceComponentInput {
    pub fee: Fee,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<ProductId>,
}

impl PriceComponentInput {
    pub fn new(fee: Fee, name: String) -> Self {
        Self {
            fee,
            name,
            product_id: None,
        }
    }
}
