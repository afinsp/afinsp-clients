// this file is @generated
use serde::{Deserialize, Serialize};

use super::product_id::ProductId;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ExistingProductRef {
    pub id: ProductId,
}

impl ExistingProductRef {
    pub fn new(id: ProductId) -> Self {
        Self { id }
    }
}
