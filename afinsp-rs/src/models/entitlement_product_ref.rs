// this file is @generated
use serde::{Deserialize, Serialize};

use super::product_id::ProductId;

/// Minimal reference to the product a feature belongs to.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EntitlementProductRef {
    pub id: ProductId,

    pub name: String,
}

impl EntitlementProductRef {
    pub fn new(id: ProductId, name: String) -> Self {
        Self { id, name }
    }
}
